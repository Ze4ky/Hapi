use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use std::str::FromStr;

pub fn json_to_headers(value: &Value) -> HeaderMap {
    let mut headers = HeaderMap::new();
    if let Value::Object(map) = value {
        for (key, val) in map {
            let header_name = match HeaderName::from_str(key) {
                Ok(name) => name,
                Err(_) => {
                    eprintln!("Invalid header name: {}", key);
                    continue;
                }
            };
            match val {
                Value::String(s) => {
                    if let Ok(header_val) = HeaderValue::from_str(s) {
                        headers.append(header_name, header_val);
                    } else {
                        eprintln!("Invalid header value for {}: {:?}", key, s);
                    }
                }
                Value::Number(n) => {
                    let s = n.to_string();
                    if let Ok(header_val) = HeaderValue::from_str(&s) {
                        headers.append(header_name, header_val);
                    }
                }
                Value::Bool(b) => {
                    let s = if *b { "true" } else { "false" }.to_string();
                    if let Ok(header_val) = HeaderValue::from_str(&s) {
                        headers.append(header_name, header_val);
                    }
                }
                Value::Array(arr) => {
                    for item in arr {
                        // 递归简化：只处理数组内字符串/数字/布尔
                        match item {
                            Value::String(s) => {
                                if let Ok(hv) = HeaderValue::from_str(s) {
                                    headers.append(header_name.clone(), hv);
                                }
                            }
                            Value::Number(n) => {
                                if let Ok(hv) = HeaderValue::from_str(&n.to_string()) {
                                    headers.append(header_name.clone(), hv);
                                }
                            }
                            Value::Bool(b) => {
                                let s = if *b { "true" } else { "false" };
                                if let Ok(hv) = HeaderValue::from_str(s) {
                                    headers.append(header_name.clone(), hv);
                                }
                            }
                            _ => {
                                eprintln!("Skipping non-string element in header {} array", key);
                            }
                        }
                    }
                }
                _ => {
                    eprintln!("Skipping unsupported header value for {}: {:?}", key, val);
                }
            }
        }
    } else {
        eprintln!("Input is not a JSON object");
    }
    headers
}
