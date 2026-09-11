//! Request/response debug logging, ported from `debuglog/roundtripper.go`.
//!
//! Go installs a `RoundTripper` into the HTTP client's transport. reqwest has
//! no equivalent public hook, so the same logging happens inside
//! [`crate::Config`]: attach a [`Config`] with
//! [`crate::Config::set_client_with_debug`] and every request made through
//! that config is logged, redacted and counted the same way.
//!
//! This has been proven useful for finding Starr app API payloads, and as a
//! general debug log wrapper for an integrating application.

use reqwest::header::HeaderMap;
use std::sync::Arc;
use std::time::Duration;

const MIN_REDACT_CHARS: usize = 4;

/// A callback you may use to collect statistics.
///
/// Receives the response status, request method, bytes sent, bytes received,
/// and the error message when the request never completed.
pub type Caller = Arc<dyn Fn(&str, &str, usize, usize, Option<&str>) + Send + Sync>;

/// A callback that receives each formatted debug line.
pub type Debugf = Arc<dyn Fn(&str) + Send + Sync>;

/// Config is the input data for the logger.
#[derive(Clone, Default)]
pub struct Config {
    /// This is where logs go. If not set they go to stderr.
    pub debugf: Option<Debugf>,
    /// This can be used for byte counters, but is optional otherwise.
    pub caller: Option<Caller>,
    /// Any strings in this list are replaced with `<redacted>` in the log
    /// output. Useful for hiding api keys and passwords from debug logs.
    /// Strings must be 4+ chars.
    pub redact: Vec<String>,
    /// Limit logged JSON payloads to this many bytes. 0 = unlimited.
    pub max_body: usize,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("debugf", &self.debugf.is_some())
            .field("caller", &self.caller.is_some())
            .field("redact", &self.redact.len())
            .field("max_body", &self.max_body)
            .finish()
    }
}

/// One request/response exchange handed to [`Config::log_exchange`].
pub struct Exchange<'a> {
    /// HTTP method of the request.
    pub method: &'a str,
    /// Full request URL.
    pub url: &'a str,
    /// Response status line, like `200 OK`.
    pub status: &'a str,
    /// Response headers.
    pub headers: &'a HeaderMap,
    /// Request body bytes that were sent.
    pub sent: &'a [u8],
    /// Response body bytes that were received.
    pub rcvd: &'a [u8],
    /// How long the exchange took.
    pub elapsed: Duration,
}

impl Config {
    /// Returns a config that writes to stderr.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the log sink.
    pub fn with_debugf(mut self, debugf: Debugf) -> Self {
        self.debugf = Some(debugf);
        self
    }

    /// Sets the statistics callback.
    pub fn with_caller(mut self, caller: Caller) -> Self {
        self.caller = Some(caller);
        self
    }

    /// Adds strings that get replaced with `<redacted>` in log output.
    pub fn with_redact(mut self, redact: Vec<String>) -> Self {
        self.redact = redact;
        self
    }

    /// Limits logged payloads to this many bytes.
    pub fn with_max_body(mut self, max_body: usize) -> Self {
        self.max_body = max_body;
        self
    }

    /// Writes one already-formatted line, applying redaction first.
    pub fn redact_log(&self, msg: &str) {
        let mut msg = msg.to_string();

        for redact in &self.redact {
            if redact.len() >= MIN_REDACT_CHARS {
                msg = msg.replace(redact.as_str(), "<redacted>");
            }
        }

        match &self.debugf {
            Some(debugf) => debugf(&msg),
            None => eprintln!("{msg}"),
        }
    }

    /// Logs a completed request/response pair and fires the callback.
    ///
    /// Returns the sent and received byte counts, like Go's `logRequest`.
    pub fn log_exchange(&self, exchange: Exchange<'_>) -> (usize, usize) {
        let Exchange {
            method,
            url,
            status,
            headers,
            sent,
            rcvd,
            elapsed,
        } = exchange;

        let sent_bytes = sent.len();
        let rcvd_bytes = rcvd.len();

        let mut sent_text = String::from_utf8_lossy(sent).into_owned();
        if self.max_body > 0 && sent_text.len() > self.max_body {
            sent_text = format!("{} <data truncated>", truncate(&sent_text, self.max_body));
        }

        let ctype = headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|val| val.to_str().ok())
            .unwrap_or("");

        // We only log JSON. Need something else? Ask!
        let rcvd_text = if !ctype.contains("json") {
            format!("<data not logged, content-type: {ctype}>")
        } else {
            let text = String::from_utf8_lossy(rcvd).into_owned();
            if self.max_body > 0 && text.len() > self.max_body {
                format!("{} <body truncated>", truncate(&text, self.max_body))
            } else {
                text
            }
        };

        let elapsed = round_millis(elapsed);

        if sent_bytes > 0 {
            self.redact_log(&format!(
                "Sent ({method}) {sent_bytes} bytes to {url} in {elapsed:?}: {sent_text}\n \
                 Response: {status} {rcvd_bytes} bytes\n{}{rcvd_text})",
                format_headers(headers)
            ));
        } else {
            self.redact_log(&format!(
                "Sent ({method}) to {url} in {elapsed:?}, Response: {status} {rcvd_bytes} bytes\n{}{rcvd_text}",
                format_headers(headers)
            ));
        }

        if let Some(caller) = &self.caller {
            caller(status, method, sent_bytes, rcvd_bytes, None);
        }

        (sent_bytes, rcvd_bytes)
    }

    /// Reports a request that never produced a response.
    ///
    /// Go sends this from `RoundTrip` because `Close()` will never be called.
    pub fn log_failure(&self, method: &str, sent: &[u8], err: &str) {
        if let Some(caller) = &self.caller {
            caller("000 Failed", method, sent.len(), 0, Some(err));
        }
    }
}

fn truncate(text: &str, max: usize) -> &str {
    let mut end = max.min(text.len());

    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }

    &text[..end]
}

fn round_millis(elapsed: Duration) -> Duration {
    Duration::from_millis(elapsed.as_millis() as u64)
}

fn format_headers(headers: &HeaderMap) -> String {
    let mut out = String::new();

    for (name, value) in headers {
        out.push_str(name.as_str());
        out.push_str(": ");
        out.push_str(value.to_str().unwrap_or("<binary>"));
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn redaction_replaces_long_secrets() {
        let lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = lines.clone();

        let config = Config::new().with_debugf(Arc::new(move |msg: &str| {
            sink.lock().unwrap().push(msg.to_string());
        }))
        .with_redact(vec!["supersecret".to_string(), "ab".to_string()]);

        config.redact_log("key=supersecret and ab");

        let lines = lines.lock().unwrap();
        assert_eq!(lines[0], "key=<redacted> and ab");
    }
}
