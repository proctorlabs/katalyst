use http::{header::HeaderValue, HeaderMap};

lazy_static! {
    static ref HOP_HEADERS: Vec<&'static str> = vec![
        "Connection",
        "Keep-Alive",
        "Proxy-Authenticate",
        "Proxy-Authorization",
        "Te",
        "Trailers",
        "Transfer-Encoding",
        "Upgrade",
    ];
}

pub fn strip_hop_headers(headers: &mut HeaderMap) {
    for header in HOP_HEADERS.iter() {
        headers.remove(*header);
    }
}

pub fn add_forwarding_headers(headers: &mut HeaderMap, remote_ip: &str) {
    headers.remove("X-Forwarded-For");
    headers.remove("X-Forwarded-Proto");
    headers.remove("X-Forwarded-Port");
    if let Ok(header) = HeaderValue::from_str(remote_ip) {
        headers.append("X-Forwarded-For", header);
    }
}
