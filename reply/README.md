# reply

[![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
[![GitHub license](https://img.shields.io/github/license/schneiderfelipe/getanswer)](https://github.com/schneiderfelipe/getanswer/blob/main/LICENSE)
[![Build CI](https://github.com/schneiderfelipe/getanswer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/getanswer/actions/workflows/ci.yml)
[![Changelog CI](https://github.com/schneiderfelipe/getanswer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/getanswer/blob/main/CHANGELOG.md#changelog)
[![Libraries.io `SourceRank`](https://img.shields.io/librariesio/sourcerank/cargo/reply)](https://libraries.io/cargo/reply)

> `reply` makes any command-line application a (stateless) [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).

```console
$ reply python
> print("hello")
hello
> print("world!")
world!
```

Read
the [installation](#installation)
and [usage](#usage) instructions below.

### Installation

#### From source (recommended)

Either clone the repository to your machine and install from it,
or install directly from GitHub.
Both options require [Rust and Cargo to be installed](https://rustup.rs/).

```console
# Option 1: cloning and installing from the repository
$ git clone https://github.com/schneiderfelipe/getanswer.git
$ cd answer && cargo install reply --path=reply/

# Option 2: installing directly from GitHub
$ cargo install reply --git=https://github.com/schneiderfelipe/getanswer
```

If you're looking to contribute to the project's development,
the first option is the way to go (and thank you for your interest!).
However,
if you simply want to install the development version,
the second option is likely the better choice.

### Unsafe code usage

This project forbids unsafe code usage.

License: MIT
