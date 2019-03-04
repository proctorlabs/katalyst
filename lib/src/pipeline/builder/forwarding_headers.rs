use http::header::HeaderValue;
use http::request::Parts;
use std::net::SocketAddr;

pub fn add_forwarding_headers(mut parts: Parts, remote: SocketAddr) -> Parts {
    let ip_str = remote.ip().to_string();
    parts.headers.remove("X-Forwarded-For");
    parts.headers.remove("X-Forwarded-Proto");
    parts.headers.remove("X-Forwarded-Port");
    parts
        .headers
        .append("X-Forwarded-For", HeaderValue::from_str(&ip_str).unwrap());
    parts
}
