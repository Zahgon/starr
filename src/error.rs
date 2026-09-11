//! Errors returned by this crate, ported from the sentinel errors and
//! `ReqError` type in the Go library.

use reqwest::header::HeaderMap;
use std::fmt;

/// Convenient result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// ReqError is returned when a Starr app returns an invalid status code.
///
/// It carries the response headers and body so callers can inspect what the
/// app actually said. Compare with [`Error::is_invalid_status_code`] to match
/// any status code, the way `errors.Is(err, starr.ErrInvalidStatusCode)` does.
#[derive(Debug, Clone, Default)]
pub struct ReqError {
    /// Response headers from the failed request.
    pub headers: HeaderMap,
    /// Sub error, often absent, or not useful.
    pub err: Option<String>,
    /// Error message extracted from the response body.
    pub msg: String,
    /// Property name extracted from the response body.
    pub name: String,
    /// Raw response body.
    pub body: Vec<u8>,
    /// HTTP status code. `-1` is the wildcard used for matching.
    pub code: i32,
}

impl ReqError {
    /// Returns a response header value, or `""` when it is missing.
    pub fn get(&self, key: &str) -> &str {
        self.headers
            .get(key)
            .and_then(|val| val.to_str().ok())
            .unwrap_or("")
    }

    /// Provides a custom error match facility, ported from Go's `Is` method.
    ///
    /// A target code of `-1` matches any [`ReqError`].
    pub fn is(&self, target: &ReqError) -> bool {
        self.code == target.code || target.code == -1
    }
}

impl fmt::Display for ReqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const PREFIX: &str = "invalid status code";
        const MAX_BODY: usize = 400; // arbitrary.

        let msg = if self.code >= 300 {
            format!("{PREFIX}, {} >= {}", self.code, 300)
        } else {
            format!("{PREFIX}, {} < {}", self.code, 200)
        };

        let body = String::from_utf8_lossy(&self.body);

        if !self.name.is_empty() {
            write!(f, "{msg}, {}: {}", self.name, self.msg)
        } else if !self.msg.is_empty() {
            write!(f, "{msg}, {}", self.msg)
        } else if body.len() > MAX_BODY {
            // Slice on a char boundary so multi-byte bodies cannot panic.
            let mut end = MAX_BODY;
            while !body.is_char_boundary(end) {
                end -= 1;
            }
            write!(f, "{msg}, {}", &body[..end])
        } else if !body.is_empty() {
            write!(f, "{msg}, {body}")
        } else {
            write!(f, "{msg}")
        }
    }
}

impl std::error::Error for ReqError {}

/// Errors you may receive from this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A Starr app returned a non-2xx status code.
    #[error("{0}")]
    Req(Box<ReqError>),

    /// Returned if you attempt a request with no HTTP client configured.
    #[error("http.Client must not be nil")]
    NilClient,

    /// Returned by `*_into` methods when a nil interface is provided.
    #[error("cannot unmarshal data into a nil or empty interface")]
    NilInterface,

    /// Returned if we know the API key didn't work.
    #[error("API Key may be incorrect")]
    InvalidApiKey,

    /// Returned when bad input is provided.
    #[error("request error: {0}")]
    Request(String),

    /// The underlying HTTP transport failed.
    #[error("{context}: {source}")]
    Http {
        /// What we were doing when the transport failed.
        context: String,
        /// The transport error.
        source: reqwest::Error,
    },

    /// A JSON payload could not be encoded or decoded.
    #[error("{context}: {source}")]
    Json {
        /// What we were encoding or decoding.
        context: String,
        /// The serde error.
        source: serde_json::Error,
    },

    /// An IO operation failed, usually while reading a body to upload.
    #[error("{context}: {source}")]
    Io {
        /// What we were reading or writing.
        context: String,
        /// The IO error.
        source: std::io::Error,
    },

    /// A URL could not be parsed.
    #[error("{context}: {source}")]
    Url {
        /// The URL we tried to build.
        context: String,
        /// The parse error.
        source: url::ParseError,
    },

    /// An error with extra context wrapped around it, like Go's `%w` chains.
    #[error("{context}: {source}")]
    Context {
        /// The added context.
        context: String,
        /// The wrapped error.
        source: Box<Error>,
    },

    /// Any other error raised by this crate.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Walks the error chain and returns the first [`ReqError`], if any.
    ///
    /// This is the equivalent of Go's `errors.As(err, &reqError)`.
    pub fn as_req_error(&self) -> Option<&ReqError> {
        match self {
            Error::Req(req_err) => Some(req_err),
            Error::Context { source, .. } => source.as_req_error(),
            _ => None,
        }
    }

    /// Reports whether this error is (or wraps) a bad status code.
    ///
    /// Equivalent to `errors.Is(err, starr.ErrInvalidStatusCode)`.
    pub fn is_invalid_status_code(&self) -> bool {
        self.as_req_error().is_some()
    }

    /// Reports whether this error is (or wraps) the given status code.
    pub fn is_status_code(&self, code: i32) -> bool {
        self.as_req_error().is_some_and(|err| err.code == code)
    }

    /// Wraps this error with extra context, like `fmt.Errorf("...: %w", err)`.
    pub fn context(self, context: impl Into<String>) -> Self {
        Error::Context {
            context: context.into(),
            source: Box::new(self),
        }
    }
}

/// Adds Go-style context to a [`Result`], preserving the underlying error.
pub trait ErrorContext<T> {
    /// Wraps any error with the message produced by `context`.
    fn ctx<C: Into<String>, F: FnOnce() -> C>(self, context: F) -> Result<T>;
}

impl<T> ErrorContext<T> for Result<T> {
    fn ctx<C: Into<String>, F: FnOnce() -> C>(self, context: F) -> Result<T> {
        self.map_err(|err| err.context(context()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_matches_any_code() {
        let invalid = ReqError { code: -1, ..Default::default() };
        let notfound = ReqError { code: 404, ..Default::default() };

        assert!(notfound.is(&invalid));
        assert!(!notfound.is(&ReqError { code: 500, ..Default::default() }));
    }

    #[test]
    fn message_formatting_matches_go() {
        let err = ReqError { code: 404, msg: "NotFound".into(), ..Default::default() };
        assert_eq!(err.to_string(), "invalid status code, 404 >= 300, NotFound");

        let err = ReqError {
            code: 404,
            name: "prop".into(),
            msg: "bad".into(),
            ..Default::default()
        };
        assert_eq!(err.to_string(), "invalid status code, 404 >= 300, prop: bad");

        let err = ReqError { code: 100, ..Default::default() };
        assert_eq!(err.to_string(), "invalid status code, 100 < 200");
    }

    #[test]
    fn context_preserves_req_error() {
        let err = Error::Req(Box::new(ReqError { code: 401, ..Default::default() })).context("api.Get(/thing)");
        assert!(err.is_invalid_status_code());
        assert!(err.is_status_code(401));
    }
}
