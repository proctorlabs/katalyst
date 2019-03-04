use http::request::Parts;

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

pub fn strip_hop_headers(mut parts: Parts) -> Parts {
    for header in HOP_HEADERS.iter() {
        parts.headers.remove(*header);
    }
    parts
}
