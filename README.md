# haste-cli

A small command line application to post the content of a file to a hastebin server. (Default: https://pastie.io/). Built because I needed it and I wanted to improve in Rust.

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

It uploads to https://pastie.io/documents by default, you can change this by setting the `HASTE_URL` environment variable.

```sh
haste [file]
```

# Testing

You can run test with:

```sh
cargo test
```
