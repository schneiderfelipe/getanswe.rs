# answer

[![Crates.io](https://img.shields.io/crates/v/answer)](https://crates.io/crates/answer)
[![Crates.io](https://img.shields.io/crates/l/answer)](https://github.com/schneiderfelipe/answer/blob/main/LICENSE)
[![CI](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml)
[![Changelog](https://github.com/schneiderfelipe/answer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/answer/blob/main/CHANGELOG.md#changelog)

A command-line application for `answer`ing _any_ question right from your terminal.

```console
$ echo "🌭 = 🥪?" | answer
No, a hot dog (🌭) is not the same as a sandwich (🥪).
While they both consist of bread and a filling,
a sandwich typically has separate slices of bread,
while a hot dog has a single bun that is sliced
on the top and filled with a sausage.
```

Read the [installation](#installation) and [usage](#usage) instructions below.

### Installation

#### From source

Either clone the repository to your machine and install from it,
or install directly from GitHub:

```console
# Option 1: cloning and installing from the repository
$ git clone https://github.com/schneiderfelipe/answer.git
$ cd answer && cargo install --path=answer/

# Option 2: installing directly from GitHub
$ cargo install --git=https://github.com/schneiderfelipe/answer
```

### Usage

`answer` receives user message content from the standard input
and returns assistant message content to the standard output:

```console
$ echo "Date of birth of Malcolm X?" | answer
The date of birth of Malcolm X is May 19, 1925.
```

You can identify a context for `answer` as a simple YAML file.
The file contains the initial part of a chat history.

```yaml
# birthdates.yml
messages:
  - role: system
    content: |-
      You are a date of birth checker.
      Given the name of a person,
      your job is to specify the date of birth of said person.
```

```console
$ echo "Malcolm X" | answer birthdates.yml
Malcolm X was born on May 19th, 1925.
```

### Unsafe

This project forbids unsafe code.

License: MIT
