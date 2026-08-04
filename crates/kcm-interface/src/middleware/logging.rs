use std::time::Instant;

/// Structured request log entry.
pub struct RequestLog {
    pub request_id: u64,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub duration_ms: u64,
    pub client_ip: String,
    pub user_agent: String,
}

impl RequestLog {
    pub fn new(request_id: u64, method: &str, path: &str) -> Self {
        RequestLog {
            request_id,
            method: method.to_string(),
            path: path.to_string(),
            status: 0,
            duration_ms: 0,
            client_ip: String::new(),
            user_agent: String::new(),
        }
    }

    pub fn complete(&mut self, status: u16, start: Instant) {
        self.status = status;
        self.duration_ms = start.elapsed().as_millis() as u64;
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{"request_id":{},"method":"{}","path":"{}","status":{},"duration_ms":{},"client_ip":"{}"}}"#,
            self.request_id, self.method, self.path, self.status, self.duration_ms, self.client_ip
        )
    }
}
