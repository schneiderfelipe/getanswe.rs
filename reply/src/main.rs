//! [![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
//! [![GitHub license](https://img.shields.io/github/license/schneiderfelipe/getanswe.rs)](https://github.com/schneiderfelipe/getanswe.rs/blob/main/LICENSE)
//! [![Build CI](https://github.com/schneiderfelipe/getanswe.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/getanswe.rs/actions/workflows/ci.yml)
//! [![Changelog CI](https://github.com/schneiderfelipe/getanswe.rs/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/getanswe.rs/blob/main/CHANGELOG.md#changelog)
//! [![Libraries.io `SourceRank`](https://img.shields.io/librariesio/sourcerank/cargo/reply)](https://libraries.io/cargo/reply)
//!
//! > [`reply`📩](https://crates.io/crates/reply) makes any command-line
//! > application
//! > a (stateless) [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).
//!
//! ```console
//! $ reply 'python | cowsay -f tux -n'
//! > print("Hello reply📩!")
//!  ________________
//! < Hello reply📩! >
//!  ----------------
//!    \
//!     \
//!         .--.
//!        |o_o |
//!        |:_/ |
//!       //   \ \
//!      (|     | )
//!     /'\_   _/`\
//!     \___)=(___/
//!
//! >
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
//! $ git clone https://github.com/schneiderfelipe/getanswe.rs.git
//! $ cd getanswe.rs && cargo install reply --path=reply/
//!
//! # Option 2: installing directly from GitHub
//! $ cargo install reply --git=https://github.com/schneiderfelipe/getanswe.rs
//! ```
//!
//! If you're looking to contribute to the project's development,
//! the first option is the way to go (and thank you for your interest!).
//! However,
//! if you simply want to install the development version,
//! the second option is likely the better choice.
//!
//! ## Usage
//!
//! Using this tool is simple:
//!
//! ```console
//! $ reply 'python'
//! >
//! ```
//!
//! Whatever you type in the prompt will be fed to the backend command (`python`
//! in the example). The output of the command will be displayed in the
//! terminal. For example:
//!
//! ```console
//! $ reply 'python'
//! > print("Hello " + "python")
//! Hello python
//! >
//! ```
//!
//! However,
//! there are a few things to keep in mind:
//!
//! - Only the standard output is captured. If nothing is printed, nothing will
//!   be shown.
//! - The REPL is stateless, which means that there's no memory being carried
//!   out. If you define a variable, for example, it won't be available in the
//!   next prompt.
//!
//! Here's an example:
//!
//! ```console
//! $ reply 'python'
//! > a = 2              # no output
//! > print(f"a = {a}")  # no memory
//! Traceback (most recent call last):
//!   File "<stdin>", line 1, in <module>
//! NameError: name 'a' is not defined
//! ```
//!
//! Therefore,
//! it's the responsibility of the backend application to
//!
//! - Print out results to the standard output.
//! - Implement memory (normally through a file).
//!
//! ## Unsafe code usage
//!
//! This project forbids unsafe code usage.

#![forbid(unsafe_code)]

use std::env;
use std::io::Read;
use std::io::Write;
use std::io::{self};

use clap::Parser;
use duct::Expression;
use duct_sh::sh_dangerous;
use rustyline::error::ReadlineError;
use rustyline::Cmd;
use rustyline::Config;
use rustyline::Editor;
use rustyline::KeyEvent;
use thiserror::Error;

/// reply makes any command-line application a (stateless) REPL.
///
/// This program sets up a REPL (Read-Evaluate-Print Loop)
/// that takes user input
/// and sends it to the backend application's standard input for evaluation.
/// The output content is retrieved from the application's standard output
/// and printed.
/// This loop continues until the program is terminated.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    /// The expression that will run as the backend application
    /// when user input is received.
    #[arg(value_parser = parse_expression)]
    expression: Expression,

    /// Verbosity options.
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

/// An error that came from [`Cli`].
#[derive(Debug, Error)]
enum CliError {}

/// Get an [`Expression`] by parsing.
#[allow(clippy::unnecessary_wraps)]
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

    let mut editor = Editor::with_config(Config::builder().auto_add_history(true).build())?;
    editor.set_helper(Some(()));
    editor.bind_sequence(KeyEvent::alt('\r'), Cmd::Newline);

    // let history_path = data_dir.join("history.txt");
    // if editor.load_history(&history_path).is_err() {
    //     log::warn!("No previous history found.");
    // }

    // Read until the line has at least one non-whitespace character.
    let mut readline = || {
        loop {
            let line = editor.readline("> ");
            match line {
                Ok(ref l) if !l.trim().is_empty() => break line,
                err @ Err(_) => break err,
                _ => {}
            }
        }
    };

    loop {
        match readline() {
            Ok(line) => {
                // editor.save_history(&self.history_path)?;

                let mut output = String::new();
                cli.expression
                    .unchecked()
                    .stdin_bytes(line)
                    .reader()?
                    .read_to_string(&mut output)?;

                let mut stdout = io::stdout().lock();
                writeln!(stdout, "{output}")?;
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
