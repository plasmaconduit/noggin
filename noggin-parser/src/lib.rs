//! Do not import or use this crate directly, import and use `noggin` instead.
//! See: [noggin](https://docs.rs/noggin/latest/noggin/)

mod from_header_value;
mod header_parser;

pub use from_header_value::FromHeaderValue;
pub use header_parser::Error;
pub use header_parser::HeadParser;
