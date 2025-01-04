use std::collections::HashMap;

use salvo::{http::{HeaderName, HeaderValue}, Request};

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

pub fn extract_query_string(req: &mut Request) -> String {
    let mut param_string = String::new();
    let params = req.parse_queries::<HashMap<String, String>>().unwrap();
    for (key, value) in params {
        param_string += format!("{}={}&", key, value).as_str();
    }
    if param_string.ends_with("&") {
        param_string = param_string[0..param_string.len() - 1].to_string();
    }
    param_string
}
