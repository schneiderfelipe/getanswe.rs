#![forbid(unsafe_code)]

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::{self};
use std::iter::once;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

use async_openai::types::CreateTranscriptionRequestArgs;
use async_openai::Client;
use clap::Parser;
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::FromSample;
use cpal::Sample;
use either::Either;
use rustyline::error::ReadlineError;
use rustyline::history::History;
use rustyline::Cmd;
use rustyline::Config;
use rustyline::Editor;
use rustyline::Helper;
use rustyline::KeyEvent;

#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();

    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    log::debug!("{cli:#?}");

    if let Some(line) = get_line_text()? {
        process_line(&line)?;
    }

    Ok(())
}

fn get_line_text() -> anyhow::Result<Option<String>> {
    let mut editor = Editor::with_config(Config::builder().auto_add_history(true).build())?;
    editor.set_helper(Some(()));
    editor.bind_sequence(KeyEvent::alt('\r'), Cmd::Newline);

    loop {
        match read_line(&mut editor) {
            Ok(line) => return Ok(Some(line)),
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => return Ok(None),
            err @ Err(_) => {
                err?;
            }
        }
    }
}

async fn get_line_audio() -> anyhow::Result<Option<String>> {
    let host = cpal::default_host();

    let devices = host.input_devices()?;
    let devices = if let Some(device) = host.default_input_device() {
        Either::Left(once(device).chain(devices))
    } else {
        Either::Right(devices)
    };

    let mut devices_configs = devices.filter_map(|device| {
        device
            .default_input_config()
            .ok()
            .map(|config| (device, config))
    });

    let Some((device, config)) = devices_configs.next() else {
         anyhow::bail!("Failed to get default input config");
     };

    let path = tempfile::Builder::new()
        .prefix("murmur")
        .suffix(".wav")
        .tempfile()?
        .into_temp_path();
    log::debug!("{path:#?}");
    let spec = wav_spec_from_config(&config)?;
    let writer = hound::WavWriter::create(&path, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    let writer_2 = writer.clone();

    let err_fn = move |err| {
        log::error!("an error occurred on stream: {err}");
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => {
            device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
                err_fn,
                None,
            )?
        }
        cpal::SampleFormat::I16 => {
            device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
                err_fn,
                None,
            )?
        }
        cpal::SampleFormat::I32 => {
            device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
                err_fn,
                None,
            )?
        }
        cpal::SampleFormat::F32 => {
            device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
                err_fn,
                None,
            )?
        }
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    stream.play()?;
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("should send signal on channel"))?;

    rx.recv()?;
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize()?;

    let response = Client::default()
        .with_api_key(env::var("OPENAI_API_KEY")?)
        .audio()
        .transcribe(
            CreateTranscriptionRequestArgs::default()
                .model("whisper-1")
                .file(&path)
                .build()?,
        )
        .await?;

    log::debug!("{response:#?}");

    path.close()?;
    Ok(Some(response.text))
    // TODO: send none if empty
}

#[inline]
fn read_line<H: Helper, I: History>(editor: &mut Editor<H, I>) -> rustyline::Result<String> {
    loop {
        let line = editor.readline("💬 ");
        match line {
            Ok(ref l) if !l.trim().is_empty() => break line,
            err @ Err(_) => break err,
            _ => {}
        }
    }
}

#[inline]
fn process_line(line: &str) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    writeln!(stdout, "GOT: {line}")?;
    stdout.flush()?;
    Ok(())
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> anyhow::Result<hound::WavSpec> {
    let wav_spec = hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: u16::try_from(config.sample_format().sample_size() * 8)?,
        sample_format: sample_format(config.sample_format()),
    };
    Ok(wav_spec)
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
