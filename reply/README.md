# reply

[![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
[![GitHub license](https://img.shields.io/github/license/schneiderfelipe/answer)](https://github.com/schneiderfelipe/answer/blob/main/LICENSE)
[![Build CI](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml)
[![Changelog CI](https://github.com/schneiderfelipe/answer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/answer/blob/main/CHANGELOG.md#changelog)
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
$ git clone https://github.com/schneiderfelipe/answer.git
$ cd answer && cargo install reply --path=reply/

# Option 2: installing directly from GitHub
$ cargo install reply --git=https://github.com/schneiderfelipe/answer
```

### Unsafe code usage

This project forbids unsafe code usage.

License: MIT
