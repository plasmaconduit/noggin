//! `noggin` - A Rust library for HTTP header parsing
//!
//! This library provides traits and derive macros to make HTTP header parsing in Rust
//! a breeze. It abstracts away the raw string manipulations, letting developers work
//! with more expressive and type-safe data structures.
//!
//! # Features
//!
//! - **Declarative**: Define your HTTP headers using Rust structs with strongly typed
//!   header values.
//! - **Zero-copy capture**: Opt-in zero-copy header value parsing.
//! - **Extensible**: Easily add new strongly typed header values.
//!
//! # Examples
//!
//! Define your HTTP header structure and derive the parsing logic using `noggin::Noggin`:
//!
//! ```rust
//! use noggin::{Noggin, HeadParser};
//!
//! #[derive(Noggin)]
//! pub struct TestHeaders<'a> {
//!     pub content_type: &'a str,
//!     pub content_length: u32,
//!     pub accept: Vec<&'a str>,
//!     pub connection: Option<&'a str>,
//!     pub pragma: Option<Vec<&'a str>>,
//! }
//!
//! let raw_headers = b"content-type: text/html\r\n\
//! content-length: 12\r\n\
//! accept: text/html, text/plain\r\n\
//! pragma: no-cache, public\r\n\
//! accept: application/json\r\n\r\n\
//! hello world!";
//!
//! let (parsed_headers, body) = TestHeaders::parse_headers(raw_headers).unwrap();
//! assert_eq!(parsed_headers.content_type, "text/html");
//! assert_eq!(parsed_headers.content_length, 12);
//! assert_eq!(parsed_headers.accept, vec!["text/html", "text/plain", "application/json"]);
//! assert_eq!(parsed_headers.pragma.unwrap(), vec!["no-cache", "public"]);
//! assert_eq!(body, b"hello world!");
//! ```

pub use noggin_derive::*;
pub use noggin_parser::*;
