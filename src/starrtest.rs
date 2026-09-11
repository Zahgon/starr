//! Test helpers shared by the app module tests, ported from `starrtest/`.
//!
//! Go uses `httptest.Server` and asserts inside the handler. Rust tests cannot
//! carry a `*testing.T` into a spawned task, so this server records what it
//! saw and the test asserts afterwards with [`MockServer::assert_request`].

use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

/// The error body for a 401 response.
pub const BODY_UNAUTHORIZED: &str = r#"{"error": "Unauthorized"}"#;
/// The error body for a 404 response.
pub const BODY_NOT_FOUND: &str = r#"{"message": "NotFound"}"#;
/// The error body for a 405 response.
pub const BODY_METHOD_NOT_ALLOWED: &str = r#"{"message": "MethodNotAllowed"}"#;

/// MockData allows generic testing of HTTP inputs and outputs.
#[derive(Debug, Clone, Default)]
pub struct MockData {
    /// A name for the test.
    pub name: String,
    /// The path expected in the request, ie. `/api/v1/thing`.
    pub expected_path: String,
    /// The request body (json) expected from the caller.
    pub expected_request: String,
    /// The request method (GET/POST) expected from the caller.
    pub expected_method: String,
    /// The (json) response body returned to the caller.
    pub response_body: String,
    /// The status that gets returned to the caller.
    pub response_status: u16,
}

impl MockData {
    /// Builds mock data for a request that is expected to succeed.
    pub fn new(name: &str, method: &str, path: &str, response_body: &str) -> Self {
        Self {
            name: name.to_string(),
            expected_method: method.to_string(),
            expected_path: path.to_string(),
            response_body: response_body.to_string(),
            response_status: 200,
            expected_request: String::new(),
        }
    }

    /// Sets the request body the caller is expected to send.
    pub fn with_request(mut self, body: &str) -> Self {
        self.expected_request = body.to_string();
        self
    }

    /// Sets the response status code.
    pub fn with_status(mut self, status: u16) -> Self {
        self.response_status = status;
        self
    }

    /// Starts a mock server that answers with this data.
    pub async fn start(&self) -> MockServer {
        MockServer::start(self.clone()).await
    }
}

/// One request the mock server received.
#[derive(Debug, Clone, Default)]
pub struct Observed {
    /// Request method, eg. `GET`.
    pub method: String,
    /// Request target including the query string.
    pub path: String,
    /// Raw request body.
    pub body: String,
}

/// A running mock HTTP server. Aborts its listener when dropped.
#[derive(Debug)]
pub struct MockServer {
    /// Base URL of the server, eg. `http://127.0.0.1:53324`.
    pub url: String,
    observed: Arc<Mutex<Vec<Observed>>>,
    handle: JoinHandle<()>,
}

impl MockServer {
    /// Binds an ephemeral port and serves `data` for every request.
    pub async fn start(data: MockData) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("binding the mock server");
        let addr = listener.local_addr().expect("reading the mock server address");

        let observed: Arc<Mutex<Vec<Observed>>> = Arc::new(Mutex::new(Vec::new()));
        let recorder = observed.clone();

        let handle = tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    return;
                };

                let Some(request) = read_request(&mut stream).await else {
                    continue;
                };

                recorder.lock().unwrap().push(request);

                let body = data.response_body.as_bytes();
                let response = format!(
                    "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    data.response_status,
                    reason_phrase(data.response_status),
                    body.len(),
                );

                let _ = stream.write_all(response.as_bytes()).await;
                let _ = stream.write_all(body).await;
                let _ = stream.flush().await;
            }
        });

        Self {
            url: format!("http://{addr}"),
            observed,
            handle,
        }
    }

    /// Returns every request the server received.
    pub fn requests(&self) -> Vec<Observed> {
        self.observed.lock().unwrap().clone()
    }

    /// Asserts that exactly one request arrived and it matches `data`.
    ///
    /// This is the Rust counterpart of the assertions Go performs inside its
    /// `httptest` handler.
    pub fn assert_request(&self, data: &MockData) {
        let requests = self.requests();

        assert_eq!(
            requests.len(),
            1,
            "{}: expected exactly one request, got {}",
            data.name,
            requests.len()
        );

        let got = &requests[0];

        assert_eq!(
            got.path, data.expected_path,
            "{}: expected_path does not match the actual path",
            data.name
        );
        assert_eq!(
            got.method, data.expected_method,
            "{}: expected_method does not match the actual method",
            data.name
        );
        assert_eq!(
            got.body, data.expected_request,
            "{}: expected_request does not match body for actual request",
            data.name
        );
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

/// Reads one HTTP/1.1 request from the stream.
async fn read_request(stream: &mut tokio::net::TcpStream) -> Option<Observed> {
    let mut buf = Vec::new();
    let mut chunk = [0_u8; 4096];

    // Read until we have the full head.
    let head_end = loop {
        if let Some(pos) = find_subslice(&buf, b"\r\n\r\n") {
            break pos + 4;
        }

        let read = stream.read(&mut chunk).await.ok()?;
        if read == 0 {
            return None;
        }

        buf.extend_from_slice(&chunk[..read]);
    };

    let head = String::from_utf8_lossy(&buf[..head_end]).into_owned();
    let mut lines = head.lines();
    let mut request_line = lines.next()?.split_whitespace();

    let method = request_line.next()?.to_string();
    let path = request_line.next()?.to_string();

    let content_length = lines
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())?
        })
        .unwrap_or(0);

    // Read the rest of the body.
    let mut body = buf[head_end..].to_vec();
    while body.len() < content_length {
        let read = stream.read(&mut chunk).await.ok()?;
        if read == 0 {
            break;
        }

        body.extend_from_slice(&chunk[..read]);
    }

    body.truncate(content_length);

    Some(Observed {
        method,
        path,
        body: String::from_utf8_lossy(&body).into_owned(),
    })
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Status",
    }
}
