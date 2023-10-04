use noggin::{HeadParser, Noggin};
use rstest::rstest;

#[derive(PartialEq, Debug, Noggin)]
pub struct TestHeaders<'a> {
    pub content_type: &'a str,
    pub content_length: u32,
    pub accept: Vec<&'a str>,
    pub connection: Option<&'a str>,
    pub pragma: Option<Vec<&'a str>>,
}

#[rstest]
#[case(
    "Content-Type: application/json\r\nContent-Length: 42\r\nAccept: application/json\r\nConnection: keep-alive",
    Ok(TestHeaders {
        content_type: "application/json",
        content_length: 42,
        accept: vec!["application/json"],
        connection: Some("keep-alive"),
        pragma: None
    })
)]
#[case(
    "Content-Type: application/json\r\nContent-Length: 42\r\nAccept: application/json,text/plain\r\nPragma: no-cache, max-age=0",
    Ok(TestHeaders {
        content_type: "application/json",
        content_length: 42,
        accept: vec!["application/json", "text/plain"],
        connection: None,
        pragma: Some(vec!["no-cache", "max-age=0"])
    })
)]
#[case(
    "Content-Type: application/json\r\nContent-Length: 42\r\nAccept: application/json\r\nConnection: keep-alive\r\nAccept:text/plain",
    Ok(TestHeaders {
        content_type: "application/json",
        content_length: 42,
        accept: vec!["application/json", "text/plain"],
        connection: Some("keep-alive"),
        pragma: None
    })
)]
#[case(
    "Content-Length: 42\r\nAccept: application/json,text/plain",
    Err(noggin::Error::MissingHeader("content-type"))
)]
#[case(
    "Content-Type: application/json\r\nContent-Length: 42",
    Err(noggin::Error::MissingHeader("accept"))
)]
#[case(
    "Content-Type: application/json\r\nContent-Length: invalid\r\nAccept: application/json",
    Err(noggin::Error::InvalidHeaderValue("content-length"))
)]
#[case(
    "Content-Type: application/json\r\nContent-Length: 42\r\nAccept",
    Err(noggin::Error::MalformedHeader)
)]
fn test_noggin(#[case] input_headers: &str, #[case] expected: Result<TestHeaders, noggin::Error>) {
    let parsed = TestHeaders::parse_head_section(input_headers);
    assert_eq!(parsed, expected);
}
