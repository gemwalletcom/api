
pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}
