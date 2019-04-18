use http::header::HeaderValue;
use http::HeaderMap;

pub fn add_forwarding_headers(headers: &mut HeaderMap, remote_ip: &str) {
    headers.remove("X-Forwarded-For");
    headers.remove("X-Forwarded-Proto");
    headers.remove("X-Forwarded-Port");
    if let Ok(header) = HeaderValue::from_str(remote_ip) {
        headers.append("X-Forwarded-For", header);
    }
}
