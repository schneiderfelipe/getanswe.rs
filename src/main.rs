#![forbid(unsafe_code)]

use std::io;
use std::io::Write;

use clap::Parser;
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

    loop {
        match read_line(&mut editor) {
            Ok(line) => process_line(&line)?,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            err @ Err(_) => {
                err?;
            }
        }
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
