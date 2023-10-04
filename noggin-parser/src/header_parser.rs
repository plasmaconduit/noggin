use memchr::memmem;

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("the http head was not complete")]
    IncompleteHead,
    #[error("the http head contained non-ascii characters")]
    NonAscii,
    #[error("missing http header: {0}")]
    MissingHeader(&'static str),
    #[error("malformed http header")]
    MalformedHeader,
    #[error("invalid http header value: {0}")]
    InvalidHeaderValue(&'static str),
}

/// The `HeadParser` trait provides a way to parse HTTP headers and potentially
/// returns the parsed headers and the remaining body of an HTTP message.
///
/// This trait is intended to be automatically implemented by the `noggin::Noggin`
/// procedural macro for suitable structs. As such, users shouldn't typically
/// need to implement it manually.
pub trait HeadParser<'de>: Sized {
    /// Parse the HTTP headers from a string slice representing the head section
    /// of an HTTP message.
    ///
    /// # Parameters
    ///
    /// * `head`: A string slice containing the head section of an HTTP message.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>`: Returns the parsed headers if successful, or
    ///   an error if parsing fails.
    fn parse_head_section(head: &'de str) -> Result<Self, Error>;

    /// Parse the HTTP headers and returns both the parsed headers and the
    /// remaining body from a byte slice containing both head and body sections
    /// of an HTTP message.
    ///
    /// This function first locates the boundary between the head and body sections
    /// (denoted by the sequence `\r\n\r\n`), then validates the ASCII nature of the
    /// head, and finally calls the `parse_head_section` function to parse the headers.
    ///
    /// # Parameters
    ///
    /// * `head_and_body`: A byte slice containing both the head and body sections
    ///   of an HTTP message.
    ///
    /// # Returns
    ///
    /// * `Result<(Self, &'de [u8]), Error>`: Returns a tuple containing the parsed
    ///   headers and the remaining body if successful, or an error if parsing fails.
    fn parse_headers(head_and_body: &'de [u8]) -> Result<(Self, &'de [u8]), Error> {
        let head_end = memmem::find(head_and_body, b"\r\n\r\n").ok_or(Error::IncompleteHead)?;
        let head_bytes = &head_and_body[..head_end];
        if !head_bytes.is_ascii() {
            return Err(Error::NonAscii);
        }
        // this is safe because we just checked if the bytes contained valid
        // ascii and ascii is strict subset of utf-8
        let head = unsafe { std::str::from_utf8_unchecked(head_bytes) };
        let headers = Self::parse_head_section(head)?;
        let body = &head_and_body[head_end + 4..];
        Ok((headers, body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct SimpleHeaders {
        pub content_length: usize,
        // Add additional headers as fields here...
    }

    impl<'de> HeadParser<'de> for SimpleHeaders {
        fn parse_head_section(head: &'de str) -> Result<Self, Error> {
            // A simple parsing implementation, for illustration.
            let content_length_str = head
                .split("\r\n")
                .find(|line| line.starts_with("Content-Length:"))
                .ok_or(Error::MissingHeader("Content-Length"))?
                .split(':')
                .nth(1)
                .ok_or(Error::MalformedHeader)?
                .trim();

            let content_length = content_length_str
                .parse::<usize>()
                .map_err(|_| Error::InvalidHeaderValue("Content-Length"))?;

            Ok(SimpleHeaders {
                content_length,
                // ... initialize other fields here
            })
        }
    }

    #[test]
    fn parse_valid_head() {
        let input_head = b"Content-Length: 5\r\nAnother-Header: value\r\n\r\nBodyHere";
        let (headers, body) = SimpleHeaders::parse_headers(input_head).unwrap();

        assert_eq!(headers, SimpleHeaders { content_length: 5 });
        assert_eq!(body, b"BodyHere");
    }

    #[test]
    fn error_on_non_ascii_head() {
        let input_head = b"Content-Length: 5\r\nNon-Ascii: \x80\x81\x82\r\n\r\nBodyHere";
        let result = SimpleHeaders::parse_headers(input_head);

        assert_eq!(result, Err(Error::NonAscii));
    }

    #[test]
    fn error_on_incomplete_head() {
        let input_head = b"Content-Length: 5\r\nAnother-Header: value\r\nBodyWithoutHeadDelimiter";
        let result = SimpleHeaders::parse_headers(input_head);

        assert_eq!(result, Err(Error::IncompleteHead));
    }

    #[test]
    fn error_on_missing_header() {
        let input_head = b"Wrong-Header: 5\r\nAnother-Header: value\r\n\r\nBodyHere";
        let result = SimpleHeaders::parse_headers(input_head);

        assert_eq!(result, Err(Error::MissingHeader("Content-Length")));
    }

    #[test]
    fn error_on_invalid_header_value() {
        let input_head = b"Content-Length: invalid_value\r\nAnother-Header: value\r\n\r\nBodyHere";
        let result = SimpleHeaders::parse_headers(input_head);

        assert_eq!(result, Err(Error::InvalidHeaderValue("Content-Length")));
    }
}
