/// The `FromHeaderValue` trait provides a mechanism for parsing individual
/// HTTP header values from string slices.
///
/// Implementers of this trait can convert a raw string representation of an
/// HTTP header value into a more strongly typed representation. This trait
/// abstracts the parsing logic for specific header value types.
///
/// This trait is similar in spirit to the standard library's `FromStr` trait,
/// but specifically tailored for HTTP header values and capable of supporting
/// zero-copy parsing.
pub trait FromHeaderValue<'de>: Sized {
    /// Parses an HTTP header value from the provided string slice.
    ///
    /// # Parameters
    ///
    /// * `slice`: A string slice representing the raw value of an HTTP header.
    ///
    /// # Returns
    ///
    /// * `Option<Self>`: Returns the parsed header value if parsing is successful,
    ///   or `None` if parsing fails.
    fn parse_header_value(slice: &'de str) -> Option<Self>;
}

fn trim(string: &str) -> &str {
    string.trim_matches(' ').trim_end_matches(' ')
}

impl<'de> FromHeaderValue<'de> for bool {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        match trim(slice) {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
}

impl<'de> FromHeaderValue<'de> for u8 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for u16 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for u32 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for u64 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for u128 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for usize {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for i8 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for i16 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for i32 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for i64 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for i128 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for isize {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for f32 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for f64 {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        trim(slice).parse().ok()
    }
}

impl<'de> FromHeaderValue<'de> for &'de [u8] {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        Some(slice.as_bytes())
    }
}

impl<'de> FromHeaderValue<'de> for &'de str {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        Some(trim(slice))
    }
}

impl<'de> FromHeaderValue<'de> for String {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        Some(trim(slice).to_owned())
    }
}

impl<'de, T: FromHeaderValue<'de>> FromHeaderValue<'de> for Vec<T> {
    fn parse_header_value(slice: &'de str) -> Option<Self> {
        let mut values = vec![];
        for value in slice.split(',') {
            let parsed = T::parse_header_value(value)?;
            values.push(parsed);
        }
        Some(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::usize;

    #[rstest]
    #[case("true", Some(true))]
    #[case("false", Some(false))]
    #[case("falsey", None)]
    fn bool_test(#[case] input: &str, #[case] expected: Option<bool>) {
        assert_eq!(expected, bool::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("255", Some(255))]
    #[case("-1", None)]
    #[case("256", None)]
    fn u8_test(#[case] input: &str, #[case] expected: Option<u8>) {
        assert_eq!(expected, u8::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-1", None)]
    fn u16_test(#[case] input: &str, #[case] expected: Option<u16>) {
        assert_eq!(expected, u16::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-1", None)]
    fn u32_test(#[case] input: &str, #[case] expected: Option<u32>) {
        assert_eq!(expected, u32::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-1", None)]
    fn u64_test(#[case] input: &str, #[case] expected: Option<u64>) {
        assert_eq!(expected, u64::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-1", None)]
    fn u128_test(#[case] input: &str, #[case] expected: Option<u128>) {
        assert_eq!(expected, u128::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-1", None)]
    fn usize_test(#[case] input: &str, #[case] expected: Option<usize>) {
        assert_eq!(expected, usize::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("idk", None)]
    fn i8_test(#[case] input: &str, #[case] expected: Option<i8>) {
        assert_eq!(expected, i8::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("idk", None)]
    fn i16_test(#[case] input: &str, #[case] expected: Option<i16>) {
        assert_eq!(expected, i16::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("idk", None)]
    fn i32_test(#[case] input: &str, #[case] expected: Option<i32>) {
        assert_eq!(expected, i32::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("idk", None)]
    fn i64_test(#[case] input: &str, #[case] expected: Option<i64>) {
        assert_eq!(expected, i64::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("idk", None)]
    fn i128_test(#[case] input: &str, #[case] expected: Option<i128>) {
        assert_eq!(expected, i128::parse_header_value(input));
    }

    #[rstest]
    #[case("42", Some(42))]
    #[case("-42", Some(-42))]
    #[case("idk", None)]
    fn isize_test(#[case] input: &str, #[case] expected: Option<isize>) {
        assert_eq!(expected, isize::parse_header_value(input));
    }

    #[rstest]
    #[case("42.7", Some(42.7))]
    #[case("-42.7", Some(-42.7))]
    #[case("idk", None)]
    fn f32_test(#[case] input: &str, #[case] expected: Option<f32>) {
        assert_eq!(expected, f32::parse_header_value(input));
    }

    #[rstest]
    #[case("5.6789", Some(5.6789f64))]
    #[case("-5.6789", Some(-5.6789f64))]
    #[case("idk", None)]
    fn f64_test(#[case] input: &str, #[case] expected: Option<f64>) {
        assert_eq!(expected, f64::parse_header_value(input));
    }

    #[rstest]
    #[case("hello", Some(b"hello".as_slice()))]
    #[case(" hello ", Some(b" hello ".as_slice()))]
    fn bytes_test(#[case] input: &str, #[case] expected: Option<&[u8]>) {
        assert_eq!(expected, <&[u8]>::parse_header_value(input));
    }

    #[rstest]
    #[case("hello", Some("hello"))]
    #[case(" hello ", Some("hello"))]
    fn str_test(#[case] input: &str, #[case] expected: Option<&str>) {
        assert_eq!(expected, <&str>::parse_header_value(input));
    }

    #[rstest]
    #[case("hello", Some("hello".to_owned()))]
    #[case(" hello ", Some("hello".to_owned()))]
    fn string_test(#[case] input: &str, #[case] expected: Option<String>) {
        assert_eq!(expected, String::parse_header_value(input));
    }

    #[rstest]
    #[case("1", Some(vec![1]))]
    #[case("1, 2", Some(vec![1, 2]))]
    #[case("1, 2, 3", Some(vec![1, 2, 3]))]
    #[case("idk", None)]
    fn vec_test(#[case] input: &str, #[case] expected: Option<Vec<u8>>) {
        assert_eq!(expected, Vec::<_>::parse_header_value(input));
    }
}
