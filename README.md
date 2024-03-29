# haste-cli
[![Tests](https://img.shields.io/github/actions/workflow/status/wtommyw/haste-cli/tests_and_coverage.yml?branch=master)](https://github.com/wtommyw/haste-cli/actions)
[![Coverage](https://img.shields.io/codecov/c/github/wtommyw/haste-cli?token=0F9O1MO7H6&logo=codecov)](https://codecov.io/gh/wtommyw/haste-cli)


A small command line application to post the content of a file to a hastebin server. (Default: https://pastie.io/). Built because I needed it and I wanted to improve in Rust and TDD.

# Requirements
You should have [Rust-lang](https://www.rust-lang.org/tools/install) installed.

> You shuold have OpenSSL 1.0.1, 1.0.2, or 1.1.0 with headers installed if you're using linux.

# Building

```sh
git clone https://github.com/wtommyw/haste-cli.git
cd haste-cli
cargo build --release
```

Executable will end up in `./target/release/`

# Usage

Upload a file to a haste-server, uses https://pastie.io/documents by default

```sh
haste <file> [url]
```

Download a file from any haste-server url

```sh
haste <url> <output filename>
```

# Testing

You can run test with:

```sh
cargo test
```
