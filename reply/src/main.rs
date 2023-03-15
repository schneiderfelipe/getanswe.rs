//! [![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
//! [![Crates.io license](https://img.shields.io/crates/l/reply)](https://github.com/schneiderfelipe/answer/blob/main/LICENSE)
//! [![Build CI](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml)
//! [![Changelog CI](https://github.com/schneiderfelipe/answer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/answer/blob/main/CHANGELOG.md#changelog)
//! [![Libraries.io `SourceRank`](https://img.shields.io/librariesio/sourcerank/cargo/reply)](https://libraries.io/cargo/reply)
//!
//! > `reply` _any_ question right from your terminal,
//! > using the same
//! > [large language model](https://en.wikipedia.org/wiki/Large_language_model)
//! > that powers
//! > [**`ChatGPT`**](https://chat.openai.com/chat).
//!
//! ```console
//! $ echo "🌭 = 🥪?" | reply
//! No, a hot dog (🌭) is not the same as a sandwich (🥪).
//! While they both consist of bread and a filling,
//! a sandwich typically has separate slices of bread,
//! while a hot dog has a single bun that is sliced
//! on the top and filled with a sausage.
//! ```
//!
//! Read
//! the [installation](#installation)
//! and [usage](#usage) instructions below.
//!
//! ## Installation
//!
//! ### From source (recommended)
//!
//! Either clone the repository to your machine and install from it,
//! or install directly from GitHub.
//! Both options require [Rust and Cargo to be installed](https://rustup.rs/).
//!
//! ```console
//! # Option 1: cloning and installing from the repository
//! $ git clone https://github.com/schneiderfelipe/reply.git
//! $ cd reply && cargo install --path=reply/
//!
//! # Option 2: installing directly from GitHub
//! $ cargo install --git=https://github.com/schneiderfelipe/reply
//! ```
//!
//! ## Environment Setup
//!
//! Before using `reply`,
//! you need to set up your environment to use
//! [`OpenAI`'s chat completion API](https://platform.openai.com/docs/guides/chat/chat-completions-beta)
//! (the same technology that powers `OpenAI`'s most advanced language model,
//! [`ChatGPT`](https://chat.openai.com/chat)).
//! To set up your environment,
//! you'll need to have a secret API key from `OpenAI`,
//! which can be obtained at
//! [`OpenAI`'s online platform](https://platform.openai.com/account/api-keys).
//!
//! Next,
//! set an environment variable in your shell as follows:
//!
//! ```shell
//! export OPENAI_API_KEY="sk-...a1b2"
//! ```
//!
//! ## Usage
//!
//! With your environment set up,
//! you're ready to start using
//! the command-line application.
//! Here's an example:
//!
//! ```console
//! $ echo "Date of birth of Malcolm X?" | reply
//! The date of birth of Malcolm X is May 19, 1925.
//! ```
//!
//! You can also get `reply`s in context by providing a YAML file containing
//! the initial part of a chat history.
//! For example:
//!
//! ```yaml
//! # birthdates.yml
//! messages:
//!   - role: system
//!     content: >-
//!       You are a date of birth checker.
//!       Given the name of a person,
//!       your job is to specify the date of birth of said person.
//! ```
//!
//! ```console
//! $ echo "Malcolm X" | reply birthdates.yml
//! Malcolm X was born on May 19th, 1925.
//! ```
//!
//! The file format closely resembles both
//! [`OpenAI`'s higher-level API](https://platform.openai.com/docs/guides/chat/introduction)
//! and
//! [its lower-level `ChatML` format](https://github.com/openai/openai-python/blob/main/chatml.md).
//!
//! ## Unsafe code usage
//!
//! This project forbids unsafe code usage.

#![forbid(unsafe_code)]

use std::{
    env,
    fs::File,
    io::{self, Read},
};

use async_openai::{
    error::OpenAIError,
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role},
    Client,
};
use clap::Parser;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};

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

/// A robot that replys questions in plain text.
#[derive(Debug, Default, Serialize, Deserialize)]
struct Bot {}

/// An error that came from [`Bot`].
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
    /// Reply, in the context of a [`Conversation`], to the given [`AsyncWrite`]r.
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

/// reply any question right from your terminal,
/// using the same large language model that powers `ChatGPT`.
///
/// It receives user message content from the standard input
/// and returns assistant message content to the standard output.
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

/// An error that came from [`Cli`].
#[derive(Debug, Error)]
enum CliError {
    #[error("could not perform a serialization or deserialization operation: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("could not perform an input or output operation: {0}")]
    Io(#[from] io::Error),
}

/// Get a [`Conversation`] from a file [`Path`] by parsing.
#[inline]
fn parse_conversation(path: &str) -> Result<Conversation, CliError> {
    let file = File::open(path)?;
    let conversation = Conversation::from_reader(file)?;
    Ok(conversation)
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

    let mut conversation = cli.conversation.unwrap_or_default();

    conversation.push({
        let mut content = String::new();
        tokio::io::stdin().read_to_string(&mut content).await?;

        Message::from_user(content)
    });

    Bot::default()
        .reply_to_writer(&conversation, tokio::io::stdout())
        .await?;
    Ok(())
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
