use serde_json::Value;

pub struct Response {
    status: u16,
    body: String,
    content_type: String,
}

impl Response {
    pub fn json(status: u16, body: Value) -> Self {
        Self {
            status,
            body: body.to_string(),
            content_type: "application/json".into(),
        }
    }

    pub fn text(status: u16, body: &str) -> Self {
        Self {
            status,
            body: body.to_string(),
            content_type: "text/plain".into(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            status_text(self.status),
            self.content_type,
            self.body.len(),
            self.body
        )
    }
}

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}
