use httparse::Request as HttpRequest;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub body: Option<Value>,
}

impl Request {
    pub fn parse(buffer: &[u8]) -> Option<Self> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = HttpRequest::new(&mut headers);

        let status = req.parse(buffer).ok()?;
        if !status.is_complete() {
            return None;
        }

        let body_start = status.unwrap();
        let body_str = std::str::from_utf8(&buffer[body_start..]).ok()?;

        // if it aint JSON thats your problem
        let body = serde_json::from_str(body_str).ok();

        Some(Self {
            method: req.method?.to_string(),
            path: req.path?.to_string(),
            body,
        })
    }
}
