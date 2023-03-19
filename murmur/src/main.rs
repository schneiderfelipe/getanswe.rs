//! murmur into your terminal and convert your speech to text using `OpenAI`'s Whisper API.
//!
//! Records a WAV file (roughly 3 seconds long) using the default input device and config.
//!
//! The input data is recorded to "$`CARGO_MANIFEST_DIR/recorded.wav`".

#![forbid(unsafe_code)]

use std::{
    fs::File,
    io::BufWriter,
    iter::once,
    sync::{Arc, Mutex},
};

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

fn main() -> Result<(), anyhow::Error> {
    let _opt = Cli::parse();

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

    let (device, config) = if let Some((device, config)) = devices_configs.next() {
        (device, config)
    } else {
        anyhow::bail!("Failed to get default input config");
    };

    // The WAV file we're recording to.
    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
    let spec = wav_spec_from_config(&config)?;
    let writer = hound::WavWriter::create(PATH, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        // TODO: log error
        eprintln!("an error occurred on stream: {err}");
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

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(3));
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize()?;
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
