#[derive(Debug)]
pub struct Listener {
    pub interface: String,
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            interface: "0.0.0.0:8080".to_string(),
        }
    }
}
