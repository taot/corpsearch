use anyhow::Result;
use spider::percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};

pub fn encode_url(url: &str) -> String {
    percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string()
}

pub fn decode_url(encoded_url: &str) -> Result<String> {
    let url = percent_decode_str(encoded_url).decode_utf8()?.to_string();
    Ok(url)
}
