//! [![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
//! [![GitHub license](https://img.shields.io/github/license/schneiderfelipe/answer)](https://github.com/schneiderfelipe/answer/blob/main/LICENSE)
//! [![Build CI](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml)
//! [![Changelog CI](https://github.com/schneiderfelipe/answer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/answer/blob/main/CHANGELOG.md#changelog)
//! [![Libraries.io `SourceRank`](https://img.shields.io/librariesio/sourcerank/cargo/reply)](https://libraries.io/cargo/reply)
//!
//! > `reply` makes a stateless [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) out of any command-line application.
//!
//! ```console
//! $ reply python
//! > print("hello")
//! hello
//! > print("world!")
//! world!
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
//! $ git clone https://github.com/schneiderfelipe/answer.git
//! $ cd reply && cargo install --path=reply/
//!
//! # Option 2: installing directly from GitHub
//! $ cargo install --git=https://github.com/schneiderfelipe/answer
//! ```
//!
//! ## Unsafe code usage
//!
//! This project forbids unsafe code usage.

#![forbid(unsafe_code)]

use std::{env, io};

use clap::Parser;
use duct::Expression;
use duct_sh::sh_dangerous;
use rustyline::{error::ReadlineError, Cmd, Config, Editor, KeyEvent};
use thiserror::Error;

/// reply any question right from your terminal,
/// using the same large language model that powers `ChatGPT`.
///
/// It receives user message content from the standard input
/// and returns assistant message content to the standard output.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    /// Expression to run when user input is received.
    #[arg(value_parser = parse_expression)]
    expression: Expression,

    /// Verbosity options.
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

/// An error that came from [`Cli`].
#[derive(Debug, Error)]
enum CliError {
    // #[error("could not perform a serialization or deserialization operation: {0}")]
    // Yaml(#[from] serde_yaml::Error),
    #[error("could not perform an input or output operation: {0}")]
    Io(#[from] io::Error),
}

/// Get an [`Expression`] by parsing.
#[inline]
fn parse_expression(input: &str) -> Result<Expression, CliError> {
    let expression = sh_dangerous(input);
    Ok(expression)
}

/// Our beloved main function.
fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();

    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    log::debug!("{cli:#?}");

    let config = Config::builder().auto_add_history(true).build();
    let mut editor = Editor::with_config(config)?;
    editor.set_helper(Some(()));
    editor.bind_sequence(KeyEvent::alt('\r'), Cmd::Newline);

    // let history_path = data_dir.join("history.txt");
    // if editor.load_history(&history_path).is_err() {
    //     log::warn!("No previous history found.");
    // }

    loop {
        let line = loop {
            let line = editor.readline("> ");
            match line {
                Ok(ref l) if !l.trim().is_empty() => break line,
                err @ Err(_) => break err,
                _ => {}
            }
        };
        match line {
            Ok(line) => {
                println!("{line}");
                // editor.save_history(&self.history_path)?;
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            err @ Err(_) => {
                err?;
            }
        }
    }

    Ok(())
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
