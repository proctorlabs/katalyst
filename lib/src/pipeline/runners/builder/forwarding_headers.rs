use http::header::HeaderValue;
use http::HeaderMap;
use std::net::SocketAddr;

pub fn add_forwarding_headers(headers: &mut HeaderMap, remote: SocketAddr) {
    let ip_str = remote.ip().to_string();
    headers.remove("X-Forwarded-For");
    headers.remove("X-Forwarded-Proto");
    headers.remove("X-Forwarded-Port");
    if let Ok(header) = HeaderValue::from_str(&ip_str) {
        headers.append("X-Forwarded-For", header);
    }
}
