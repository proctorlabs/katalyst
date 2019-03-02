use http::request::Parts;

pub fn add_forwarding_headers(parts: Parts) -> Parts {
    warn!("Forwarding headers not currently implemented...");
    //TODO: Remote connection info not available without overriding some parts of hyper listener...
    parts
}
