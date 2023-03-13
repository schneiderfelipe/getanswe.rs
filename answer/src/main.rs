//! A command-line application for answering _any_ question right from your terminal.
//!
//! ```console
//! $ echo "🌭 = 🥪?" | answer
//! No, a hot dog (🌭) is not the same as a sandwich (🥪).
//! While they both consist of bread and a filling,
//! a sandwich typically has separate slices of bread,
//! while a hot dog has a single bun that is sliced
//! on the top and filled with a sausage.
//! ```

use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use anyhow::{ensure, Context};
use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role},
    Client,
};
use clap::Parser;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

/// A command-line application for answering any question right from your terminal.
///
/// It receives a user message in plain text from the standard input
/// and returns an assistant message in plain text to the standard output.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    /// Path to a conversation YAML file.
    #[arg(value_parser = parse_conversation)]
    conversation: Option<Conversation>,

    /// Verbosity options.
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

/// The context of a conversation.
///
/// It can be used for building prompts or storing chat history.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Conversation {
    /// [`Message`]s in this [`Conversation`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    messages: Vec<Message>,
}

impl Conversation {
    /// Append a new [`Message`] to the end of this [`Conversation`].
    #[inline]
    fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Parse a [`Conversation`] from a [`Read`]er.
    #[inline]
    fn from_reader<R>(reader: R) -> Result<Self, serde_yaml::Error>
    where
        R: Read,
    {
        serde_yaml::from_reader(reader)
    }
}

/// A [`Conversation`] message.
///
/// This is basically a redefinition of [`ChatCompletionRequestMessage`]
/// so that we can implement new traits and methods.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Message {
    /// The [`Role`] of the author of the [`Message`].
    #[serde(default, skip_serializing_if = "is_user")]
    role: Role,
    /// The contents of the [`Message`].
    #[serde(default, skip_serializing_if = "String::is_empty")]
    content: String,
    /// The name of the author in a multi-agent [`Conversation`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Message {
    /// Create a [`Message`] whose [`Role`] is user.
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
    /// Convert a [`Message`] into a [`ChatCompletionRequestMessage`].
    #[inline]
    fn from(message: Message) -> Self {
        Self {
            role: message.role,
            content: message.content,
            name: message.name,
        }
    }
}

/// A robot that answers questions in plain text.
#[derive(Debug, Default, Serialize, Deserialize)]
struct Bot {}

impl Bot {
    /// Reply, in the context of a [`Conversation`], to the given [`Write`]r.
    #[inline]
    async fn reply_to_writer<W>(
        &self,
        conversation: &Conversation,
        mut writer: W,
    ) -> anyhow::Result<()>
    where
        W: Write,
    {
        let mut stream = Client::default()
            .with_api_key(env::var("OPENAI_API_KEY")?)
            .chat()
            .create_stream({
                CreateChatCompletionRequestArgs::default()
                    .model("gpt-3.5-turbo")
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
            response?
                .choices
                .into_iter()
                .filter_map(|choice| choice.delta.content)
                .try_for_each(|content| write!(writer, "{content}"))?;
        }

        Ok(())
    }
}

/// Our beloved main function.
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();

    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    log::debug!("{cli:#?}");

    let mut conversation = cli.conversation.unwrap_or_default();

    conversation.push({
        let mut content = String::new();
        io::stdin().lock().read_to_string(&mut content)?;

        Message::from_user(content)
    });

    Bot::default()
        .reply_to_writer(&conversation, io::stdout().lock())
        .await?;
    Ok(())
}

/// Get a [`Conversation`] from a file [`Path`] by parsing.
#[inline]
fn parse_conversation(path: &str) -> anyhow::Result<Conversation> {
    let path = Path::new(path);
    ensure!(
        path.extension().and_then(OsStr::to_str) == Some("yml"),
        "{path} should end with .yml",
        path = path.display()
    );
    ensure!(
        path.try_exists()?,
        "{path} does not exist",
        path = path.display()
    );

    let file = File::open(path)
        .with_context(|| format!("{path} could not be open", path = path.display()))?;
    let conversation = Conversation::from_reader(file)
        .with_context(|| format!("{path} could not be parsed", path = path.display()))?;

    Ok(conversation)
}

/// Determine whether a [`Role`] corresponds to a user.
#[inline]
const fn is_user(role: &Role) -> bool {
    match role {
        Role::User => true,
        Role::System | Role::Assistant => false,
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
