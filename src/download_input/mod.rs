use bytes::Bytes;
use eyre::Result;
use reqwest::{
    blocking,
    header::{HeaderMap, HeaderValue, COOKIE},
};

pub fn download_input(token: &str, year: u16, day: u8) -> Result<Bytes> {
    let mut headers = HeaderMap::new();
    headers.append(COOKIE, HeaderValue::from_str(&format!("session={token}"))?);

    let client = blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    let response = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .send()?;

    Ok(response.bytes()?)
}
