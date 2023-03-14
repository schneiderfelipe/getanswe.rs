# answer

[![Crates.io](https://img.shields.io/crates/v/answer)](https://crates.io/crates/answer)
[![Crates.io](https://img.shields.io/crates/l/answer)](https://github.com/schneiderfelipe/answer/blob/main/LICENSE)
[![CI](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml/badge.svg)](https://github.com/schneiderfelipe/answer/actions/workflows/ci.yml)
[![Changelog](https://github.com/schneiderfelipe/answer/actions/workflows/changelog.yml/badge.svg)](https://github.com/schneiderfelipe/answer/blob/main/CHANGELOG.md#changelog)

A command-line application for answering _any_ question right from your terminal.

```console
$ echo "🌭 = 🥪?" | answer
No, a hot dog (🌭) is not the same as a sandwich (🥪).
While they both consist of bread and a filling,
a sandwich typically has separate slices of bread,
while a hot dog has a single bun that is sliced
on the top and filled with a sausage.
```

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

License: MIT
