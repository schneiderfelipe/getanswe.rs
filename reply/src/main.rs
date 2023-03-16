//! [![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
//! [![GitHub license](https://img.shields.io/github/license/schneiderfelipe/getanswer)](https://github.com/schneiderfelipe/getanswer/blob/main/LICENSE)
//! [![Build CI](https://github.com/schneiderfelipe/getanswer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/getanswer/actions/workflows/ci.yml)
//! [![Changelog CI](https://github.com/schneiderfelipe/getanswer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/getanswer/blob/main/CHANGELOG.md#changelog)
//! [![Libraries.io `SourceRank`](https://img.shields.io/librariesio/sourcerank/cargo/reply)](https://libraries.io/cargo/reply)
//!
//! > `reply` makes any command-line application a (stateless) [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).
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
//! $ git clone https://github.com/schneiderfelipe/getanswer.git
//! $ cd answer && cargo install reply --path=reply/
//!
//! # Option 2: installing directly from GitHub
//! $ cargo install reply --git=https://github.com/schneiderfelipe/getanswer
//! ```
//!
//! If you're looking to contribute to the project's development,
//! the first option is the way to go (and thank you for your interest!).
//! However,
//! if you simply want to install the development version,
//! the second option is likely the better choice.
//!
//! ## Unsafe code usage
//!
//! This project forbids unsafe code usage.

#![forbid(unsafe_code)]

use std::{
    env,
    io::{self, Read, Write},
};

use clap::Parser;
use duct::Expression;
use duct_sh::sh_dangerous;
use rustyline::{error::ReadlineError, Cmd, Config, Editor, KeyEvent};
use thiserror::Error;

// TODO: review from here
/// reply makes any command-line application a (stateless) REPL.
///
/// It builds a REPL that feeds user input to the standard input
/// of backend application,
/// and gets back output content from the standard output of it.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    /// Expression to run as the backend when user input is received.
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
                // editor.save_history(&self.history_path)?;

                let mut reader = cli.expression.unchecked().stdin_bytes(line).reader()?;

                let mut output = String::new();
                reader.read_to_string(&mut output)?;

                let mut stdout = io::stdout().lock();
                write!(stdout, "{output}")?;
                stdout.flush()?;
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
