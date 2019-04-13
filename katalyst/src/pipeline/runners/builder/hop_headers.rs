use http::HeaderMap;

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
