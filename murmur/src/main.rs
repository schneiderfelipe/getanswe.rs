//! murmur into your terminal and convert your speech to text using `OpenAI`'s Whisper API.
//!
//! Records a WAV file using the default input device and config until the user indicates end of input.
//!
//! The input data is recorded to "$`CARGO_MANIFEST_DIR/recorded.wav`".
//!
//! ## Installation
//!
//! Note: if you're using [ALSA](https://www.alsa-project.org/wiki/Main_Page),
//! you may need to install the development files for `libasound2`,
//!
//! ```console
//! $ sudo apt install libasound2-dev
//! ```

#![forbid(unsafe_code)]

use std::{
    env,
    fs::File,
    io::{self, BufWriter, Write},
    iter::once,
    sync::{mpsc::channel, Arc, Mutex},
};

use async_openai::{types::CreateTranscriptionRequestArgs, Client};
use clap::Parser;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample,
};
use either::Either;

/// murmur into your terminal and convert your speech to text using `OpenAI`'s Whisper API.
///
/// The program will continue recording until you signal "end of file" (Ctrl-D),
/// and then it will output the transcribed text to the standard output.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    /// Verbosity options.
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

/// Our beloved main function.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();

    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    log::debug!("{cli:#?}");

    let host = cpal::default_host();

    // Set up the input devices and stream with the default input configs.
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

    // The WAV file we're recording to.
    let path = tempfile::Builder::new()
        .prefix("murmur")
        .suffix(".wav")
        .tempfile()?
        .into_temp_path();
    log::debug!("{path:#?}");
    let spec = wav_spec_from_config(&config)?;
    let writer = hound::WavWriter::create(&path, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        log::error!("an error occurred on stream: {err}");
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
            err_fn,
            None,
        )?,
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
    writeln!(io::stdout().lock(), "{text}", text = response.text)?;

    path.close()?;
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
