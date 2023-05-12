# reply

[![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
[![GitHub license](https://img.shields.io/github/license/schneiderfelipe/getanswe.rs)](https://github.com/schneiderfelipe/getanswe.rs/blob/main/LICENSE)
[![Build CI](https://github.com/schneiderfelipe/getanswe.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/getanswe.rs/actions/workflows/ci.yml)
[![Changelog CI](https://github.com/schneiderfelipe/getanswe.rs/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/getanswe.rs/blob/main/CHANGELOG.md#changelog)
[![Libraries.io `SourceRank`](https://img.shields.io/librariesio/sourcerank/cargo/reply)](https://libraries.io/cargo/reply)

> [`reply`📩](https://crates.io/crates/reply) makes any command-line
> application
> a (stateless) [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).

```console
$ reply 'python | cowsay -f tux -n'
> print("Hello reply📩!")
 ________________
< Hello reply📩! >
 ----------------
   \
    \
        .--.
       |o_o |
       |:_/ |
      //   \ \
     (|     | )
    /'\_   _/`\
    \___)=(___/

>
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
$ git clone https://github.com/schneiderfelipe/getanswe.rs.git
$ cd getanswe.rs && cargo install reply --path=reply/

# Option 2: installing directly from GitHub
$ cargo install reply --git=https://github.com/schneiderfelipe/getanswe.rs
```

If you're looking to contribute to the project's development,
the first option is the way to go (and thank you for your interest!).
However,
if you simply want to install the development version,
the second option is likely the better choice.

### Usage

Using this tool is simple:

```console
$ reply 'python'
>
```

Whatever you type in the prompt will be fed to the backend command (`python`
in the example). The output of the command will be displayed in the
terminal. For example:

```console
$ reply 'python'
> print("Hello " + "python")
Hello python
>
```

However,
there are a few things to keep in mind:

- Only the standard output is captured. If nothing is printed, nothing will
  be shown.
- The REPL is stateless, which means that there's no memory being carried
  out. If you define a variable, for example, it won't be available in the
  next prompt.

Here's an example:

```console
$ reply 'python'
> a = 2              # no output
> print(f"a = {a}")  # no memory
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
NameError: name 'a' is not defined
```

Therefore,
it's the responsibility of the backend application to

- Print out results to the standard output.
- Implement memory (normally through a file).

### Unsafe code usage

This project forbids unsafe code usage.

License: MIT
