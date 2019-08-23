# haste-cli

A small command line application to post the content of a file to a hastebin server. (Default: https://hasteb.in). Built because I needed it and I wanted to improve in Rust.

# Requirements
You should have [Rust-lang](https://www.rust-lang.org/tools/install) installed.

# Building

```sh
git clone https://github.com/wtommyw/haste-cli.git
cd haste-cli
cargo build --release
```

Executable will end up in `./target/release/`

# Usage

```sh
hst test.txt
```

# Testing

You can run test with:

```sh
cargo test
```
