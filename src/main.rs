#![forbid(unsafe_code)]

use std::env;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::iter::once;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

use async_openai::error::OpenAIError;
use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::CreateTranscriptionRequestArgs;
use async_openai::types::Role;
use async_openai::Client;
use clap::Parser;
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::FromSample;
use cpal::Sample;
use either::Either;
use futures::StreamExt;
use rustyline::error::ReadlineError;
use rustyline::history::History;
use rustyline::Cmd;
use rustyline::Config;
use rustyline::Editor;
use rustyline::Helper;
use rustyline::KeyEvent;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(value_parser = parse_conversation)]
    conversation: Option<Conversation>,

    #[arg(short, long)]
    record: bool,

    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Error)]
enum CliError {
    #[error("could not perform a serialization or deserialization operation: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("could not perform an input or output operation: {0}")]
    Io(#[from] io::Error),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();

    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    log::debug!("{cli:#?}");

    let line = if cli.record {
        get_line_audio().await?
    } else {
        get_line_text().await?
    };

    if let Some(line) = line {
        let conversation = cli.conversation.unwrap_or_default();
        process_line(conversation, &line).await?;
    }

    Ok(())
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Conversation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    messages: Vec<Message>,
}

impl Conversation {
    #[inline]
    fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    #[inline]
    fn from_reader<R>(reader: R) -> Result<Self, serde_yaml::Error>
    where
        R: Read,
    {
        serde_yaml::from_reader(reader)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Message {
    #[serde(default, skip_serializing_if = "is_user")]
    role: Role,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Message {
    #[inline]
    fn from_user<C>(content: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            role: Role::User,
            content: content.into(),
            name: None,
        }
    }
}

impl From<Message> for ChatCompletionRequestMessage {
    #[inline]
    fn from(message: Message) -> Self {
        Self {
            role: message.role,
            content: message.content,
            name: message.name,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Bot {}

#[derive(Debug, Error)]
enum BotError {
    #[error("could not obtain environment variable: {0}")]
    Var(#[from] env::VarError),
    #[error("could not exchange data with OpenAI: {0}")]
    OpenAI(#[from] OpenAIError),
    #[error("could not perform an input or output operation: {0}")]
    Io(#[from] io::Error),
}

impl Bot {
    #[inline]
    async fn reply_to_writer<W>(
        &self,
        conversation: &Conversation,
        mut writer: W,
    ) -> Result<(), BotError>
    where
        W: AsyncWrite + Send + Unpin,
    {
        let mut stream = Client::default()
            .with_api_key(env::var("OPENAI_API_KEY")?)
            .chat()
            .create_stream({
                CreateChatCompletionRequestArgs::default()
                    .model("gpt-3.5-turbo")
                    .temperature(0.0)
                    .messages(
                        conversation
                            .messages
                            .iter()
                            .cloned()
                            .map(Into::into)
                            .collect::<Vec<_>>(),
                    )
                    .build()?
            })
            .await?;

        while let Some(response) = stream.next().await {
            for content in response?
                .choices
                .into_iter()
                .filter_map(|choice| choice.delta.content)
            {
                writer.write_all(content.as_bytes()).await?;
            }

            writer.flush().await?;
        }

        Ok(())
    }
}

#[inline]
fn parse_conversation(path: &str) -> Result<Conversation, CliError> {
    let file = File::open(path)?;
    let conversation = Conversation::from_reader(file)?;
    Ok(conversation)
}

#[inline]
const fn is_user(role: &Role) -> bool {
    match role {
        Role::User => true,
        Role::System | Role::Assistant => false,
    }
}

async fn get_line_text() -> anyhow::Result<Option<String>> {
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
    let mut stdout = io::stdout().lock();
    write!(stdout, "🔴 (Ctrl-C to stop recording)")?;
    stdout.flush()?;

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
    writeln!(stdout)?;

    if response.text.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(response.text))
    }
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
async fn process_line(mut conversation: Conversation, line: &str) -> anyhow::Result<()> {
    conversation.push(Message::from_user(line));

    Bot::default()
        .reply_to_writer(&conversation, tokio::io::stdout())
        .await?;
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
