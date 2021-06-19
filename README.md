# node2text [![Crates.io](https://img.shields.io/crates/v/node2text)](https://crates.io/crates/node2text) ![License](https://img.shields.io/crates/l/node2text)
> A tool to extract text from HTML from your terminal.

# Usage

```bash
# pipe in
curl -s 'https://en.wikipedia.org/wiki/Wiki' | node2text '#siteSub'
# Outputs: From Wikipedia, the free encyclopedia

# extract from path
node2text '#app.title' /path/to/file.html
# May or may not output depending on if selector is matched
```

# Motivation

When I reinstall my machine, I want to automate my install process. Usually it involves quickly grabbing snippet from the internet and writing it to file, this tool aims to help script it.

Hugely inspired by [pup](https://github.com/ericchiang/pup).

# Demo

[![demo](./assets/demo.svg)](./assets/demo.svg)

# Installation

If you have rust toolchain installed, `node2text` is available on [crates.io](https://crates.io/crates/node2text), if you don't have rust toolchain installed, please install rust by going to the [official website](https://www.rust-lang.org/tools/install).

Run

```bash
cargo install node2text
```

# Note

Piping will always take precedence even if `<path>` is provided.

# Comparison with [pup](https://github.com/ericchiang/pup):

`node2text`
- Selectors are purely CSS selectors, no dsl
- Takes html, spits out text
- Written in rust programming language
- Less features than `pup`
- Outputs are not escaped

`pup`
- Selectors are CSS selectors plus dsl
- Takes html, spits out text, json, html
- Written in go programming language
- Has many features, visit their github page to know more
- Outputs are escaped
