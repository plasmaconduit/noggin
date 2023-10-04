# Noggin

[![crates.io][crates-badge]][crates-link]
[![docs.rs][docs-badge]][docs-link]

[crates-badge]: https://img.shields.io/crates/v/noggin
[crates-link]: https://crates.io/crates/noggin
[docs-badge]: https://img.shields.io/docsrs/noggin
[docs-link]: https://docs.rs/noggin/latest/noggin/

**A declarative, zero-copy, proc-macro based header parser for Rust.**

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [Features](#features)
- [Examples](#examples)
- [Testing](#testing)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Features

- **Declarative**: Define your HTTP headers using Rust structs with strongly typed header values.
- **Zero-copy capture**: Opt-in zero-copy header value parsing.
- **Extensible**: Easily add new strongly typed header values.

## Examples

Define your HTTP header structure and derive the parsing logic using `noggin::Noggin`:

```rust
use noggin::{Noggin, HeadParser};

#[derive(Noggin)]
pub struct TestHeaders<'a> {
    pub content_type: &'a str,
    pub content_length: u32,
    pub accept: Vec<&'a str>,
    pub connection: Option<&'a str>,
    pub pragma: Option<Vec<&'a str>>,
}

let raw_headers = b"content-type: text/html\r\n\
content-length: 12\r\n\
accept: text/html, text/plain\r\n\
pragma: no-cache, public\r\n\
accept: application/json\r\n\r\n\
hello world!";

let (parsed_headers, body) = TestHeaders::parse_headers(raw_headers).unwrap();
assert_eq!(parsed_headers.content_type, "text/html");
assert_eq!(parsed_headers.content_length, 12);
assert_eq!(parsed_headers.accept, vec!["text/html", "text/plain", "application/json"]);
assert_eq!(parsed_headers.pragma.unwrap(), vec!["no-cache", "public"]);
assert_eq!(body, b"hello world!");
```

## Testing

Tests should run fine with the standard `cargo test`.

However, for consistency, we recommend using the dockerized test environment.
To use the dockerized test environment the only requirements are `make` and
`docker` (you don't even need rust installed locally). Simply run the
following command.

```
make test
```
