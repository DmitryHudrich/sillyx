use std::collections::HashMap;

use salvo::http::{HeaderName, HeaderValue};

pub fn sep_headers(response: impl Into<String>) -> (String, String) {
    let response = response.into();
    if let Some(header_body_split) = response.split_once("\r\n\r\n") {
        let headers = header_body_split.0.to_string();
        let body = header_body_split.1.to_string();
        (headers, body)
    } else {
        (response.to_string(), String::new())
    }
}

pub fn parse_headers(header_text: String) -> HashMap<HeaderName, HeaderValue> {
    let mut headers = HashMap::new();
    let split = header_text.split("\r\n");
    for line in split {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.parse().expect("header_name is strange."), value.parse().expect("header value is strange"));
        }
    }

    headers
}

