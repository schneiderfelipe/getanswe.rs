# reply

[![Crates.io version](https://img.shields.io/crates/v/reply)](https://crates.io/crates/reply)
[![Crates.io license](https://img.shields.io/crates/l/reply)](https://github.com/schneiderfelipe/answer/blob/main/LICENSE)
[![Build CI](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml)
[![Changelog CI](https://github.com/schneiderfelipe/answer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/answer/blob/main/CHANGELOG.md#changelog)
[![Libraries.io `SourceRank`](https://img.shields.io/librariesio/sourcerank/cargo/reply)](https://libraries.io/cargo/reply)

> `reply` _any_ question right from your terminal,
> using the same
> [large language model](https://en.wikipedia.org/wiki/Large_language_model)
> that powers
> [**`ChatGPT`**](https://chat.openai.com/chat).

```console
$ echo "🌭 = 🥪?" | reply
No, a hot dog (🌭) is not the same as a sandwich (🥪).
While they both consist of bread and a filling,
a sandwich typically has separate slices of bread,
while a hot dog has a single bun that is sliced
on the top and filled with a sausage.
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
$ git clone https://github.com/schneiderfelipe/reply.git
$ cd reply && cargo install --path=reply/

# Option 2: installing directly from GitHub
$ cargo install --git=https://github.com/schneiderfelipe/reply
```

### Environment Setup

Before using `reply`,
you need to set up your environment to use
[`OpenAI`'s chat completion API](https://platform.openai.com/docs/guides/chat/chat-completions-beta)
(the same technology that powers `OpenAI`'s most advanced language model,
[`ChatGPT`](https://chat.openai.com/chat)).
To set up your environment,
you'll need to have a secret API key from `OpenAI`,
which can be obtained at
[`OpenAI`'s online platform](https://platform.openai.com/account/api-keys).

Next,
set an environment variable in your shell as follows:

```shell
export OPENAI_API_KEY="sk-...a1b2"
```

### Usage

With your environment set up,
you're ready to start using
the command-line application.
Here's an example:

```console
$ echo "Date of birth of Malcolm X?" | reply
The date of birth of Malcolm X is May 19, 1925.
```

You can also get `reply`s in context by providing a YAML file containing
the initial part of a chat history.
For example:

```yaml
# birthdates.yml
messages:
  - role: system
    content: >-
      You are a date of birth checker.
      Given the name of a person,
      your job is to specify the date of birth of said person.
```

```console
$ echo "Malcolm X" | reply birthdates.yml
Malcolm X was born on May 19th, 1925.
```

The file format closely resembles both
[`OpenAI`'s higher-level API](https://platform.openai.com/docs/guides/chat/introduction)
and
[its lower-level `ChatML` format](https://github.com/openai/openai-python/blob/main/chatml.md).

### Unsafe code usage

This project forbids unsafe code usage.

License: MIT
