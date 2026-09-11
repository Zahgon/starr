//! Request building and path handling, ported from `http.go`.

use crate::error::{Error, ReqError, Result};
use crate::values::Values;
use reqwest::header::HeaderMap;
use serde::Serialize;
use std::fmt;

/// API is the beginning of every API path.
pub const API: &str = "api";

/// Request contains the GET and/or POST values for an HTTP request.
#[derive(Debug, Clone, Default)]
pub struct Request {
    /// Used in PUT, POST, DELETE. Not for GET.
    pub body: Option<Vec<u8>>,
    /// GET parameters work for any request type.
    pub query: Option<Values>,
    /// Required: path portion of the URL.
    pub uri: String,
    /// Optional extra HTTP headers merged after defaults
    /// (overrides Content-Type, Accept, etc.).
    pub headers: Option<HeaderMap>,
}

impl Request {
    /// Creates a request for the given URI path.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            ..Default::default()
        }
    }

    /// Attaches a raw request body.
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Serializes `value` as JSON and attaches it as the request body.
    ///
    /// This replaces the `json.NewEncoder(&body).Encode(v)` dance repeated
    /// throughout the Go library.
    pub fn with_json<T: Serialize + ?Sized>(mut self, value: &T) -> Result<Self> {
        let body = serde_json::to_vec(value).map_err(|source| Error::Json {
            context: format!("json.Marshal({})", self.uri),
            source,
        })?;

        self.body = Some(body);

        Ok(self)
    }

    /// Attaches query parameters.
    pub fn with_query(mut self, query: Values) -> Self {
        self.query = Some(query);
        self
    }

    /// Attaches extra headers, which override the defaults.
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }
}

impl fmt::Display for Request {
    /// Turns a request into a string. Usually used in error messages.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.uri)
    }
}

/// Joins path elements and cleans the result, like Go's `path.Join`.
pub fn path_join(elements: &[&str]) -> String {
    let joined = elements
        .iter()
        .filter(|el| !el.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join("/");

    if joined.is_empty() {
        return String::new();
    }

    path_clean(&joined)
}

/// Cleans a slash-separated path, like Go's `path.Clean`.
///
/// Collapses duplicate separators, resolves `.` and `..`, and removes a
/// trailing slash unless the result is the root.
pub fn path_clean(path: &str) -> String {
    if path.is_empty() {
        return ".".to_string();
    }

    let rooted = path.starts_with('/');
    let mut out: Vec<&str> = Vec::new();

    for segment in path.split('/') {
        match segment {
            "" | "." => continue,
            ".." => {
                if let Some(last) = out.last()
                    && *last != ".."
                {
                    out.pop();
                    continue;
                }

                if !rooted {
                    out.push("..");
                }
            }
            other => out.push(other),
        }
    }

    let body = out.join("/");

    match (rooted, body.is_empty()) {
        (true, _) => format!("/{body}"),
        (false, true) => ".".to_string(),
        (false, false) => body,
    }
}

/// Makes sure the path starts with `/api`.
pub fn set_api_path(uri_path: &str) -> String {
    let api_slash = format!("{API}/");
    let rooted_api_slash = format!("/{API}/");

    if uri_path.starts_with(&api_slash) || uri_path.starts_with(&rooted_api_slash) {
        return path_join(&["/", uri_path]);
    }

    path_join(&["/", API, uri_path])
}

/// Attempts to extract an error message from a non-2xx response body.
///
/// Mirrors Go's `parseNon200`: try `{"message":...}`, then
/// `{"errorMessage":..., "propertyName":...}`, then a list of the latter.
pub fn parse_non_200(code: i32, headers: HeaderMap, body: Vec<u8>) -> ReqError {
    #[derive(serde::Deserialize)]
    struct Msg {
        #[serde(default)]
        message: String,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PropError {
        #[serde(default)]
        error_message: String,
        #[serde(default)]
        property_name: String,
    }

    let mut response = ReqError {
        headers,
        code,
        body,
        ..Default::default()
    };

    match serde_json::from_slice::<Msg>(&response.body) {
        Ok(msg) if !msg.message.is_empty() => {
            response.msg = msg.message;
            return response;
        }
        Ok(_) => {}
        Err(err) => response.err = Some(err.to_string()),
    }

    match serde_json::from_slice::<PropError>(&response.body) {
        Ok(err_msg) if !err_msg.error_message.is_empty() => {
            response.name = err_msg.property_name;
            response.msg = err_msg.error_message;
            response.err = None;
            return response;
        }
        Ok(_) => response.err = None,
        Err(err) => response.err = Some(err.to_string()),
    }

    // Sometimes we get a list of errors. This grabs the first one.
    match serde_json::from_slice::<Vec<PropError>>(&response.body) {
        Ok(list) if !list.is_empty() => {
            response.name = list[0].property_name.clone();
            response.msg = list[0].error_message.clone();
            response.err = None;
        }
        Ok(_) => response.err = None,
        Err(err) => response.err = Some(err.to_string()),
    }

    response
}

/// Builds a `multipart/form-data` body containing a single file part.
///
/// Returns the body bytes and the `Content-Type` header value, matching Go's
/// `multipart.Writer.CreateFormFile` plus `FormDataContentType()`. Used by the
/// backup-restore upload endpoints in every app.
pub fn multipart_form_file(field: &str, filename: &str, data: &[u8]) -> (Vec<u8>, String) {
    // Go's multipart writer picks a random 60-hex-char boundary; any value that
    // cannot appear in the payload works, so derive one from the content length.
    let boundary = format!("starrrustboundary{:016x}{:016x}", data.len(), field.len());

    let escape = |val: &str| val.replace('\\', "\\\\").replace('"', "\\\"");

    let mut body = Vec::new();
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
            escape(field),
            escape(filename)
        )
        .as_bytes(),
    );
    body.extend_from_slice(b"Content-Type: application/octet-stream\r\n\r\n");
    body.extend_from_slice(data);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    (body, format!("multipart/form-data; boundary={boundary}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multipart_body_has_both_delimiters() {
        let (body, ctype) = multipart_form_file("file", "backup.zip", b"data");
        let text = String::from_utf8_lossy(&body);

        assert!(ctype.starts_with("multipart/form-data; boundary="));
        assert!(text.contains(r#"name="file"; filename="backup.zip""#));
        assert!(text.ends_with("--\r\n"));
    }

    #[test]
    fn api_path_is_prefixed_once() {
        assert_eq!(set_api_path("v3/movie"), "/api/v3/movie");
        assert_eq!(set_api_path("/v3/movie"), "/api/v3/movie");
        assert_eq!(set_api_path("api/v3/movie"), "/api/v3/movie");
        assert_eq!(set_api_path("/api/v3/movie"), "/api/v3/movie");
    }

    #[test]
    fn path_join_cleans_like_go() {
        assert_eq!(path_join(&["v1/system", "status"]), "v1/system/status");
        assert_eq!(path_join(&["/", "api", "v1/tag"]), "/api/v1/tag");
        assert_eq!(path_join(&["v1/tag", "detail", "3"]), "v1/tag/detail/3");
        assert_eq!(path_clean("/a/b/../c/"), "/a/c");
    }

    #[test]
    fn parse_non_200_reads_message() {
        let err = parse_non_200(404, HeaderMap::new(), br#"{"message": "NotFound"}"#.to_vec());
        assert_eq!(err.msg, "NotFound");

        let err = parse_non_200(
            400,
            HeaderMap::new(),
            br#"[{"errorMessage":"bad","propertyName":"name"}]"#.to_vec(),
        );
        assert_eq!(err.msg, "bad");
        assert_eq!(err.name, "name");
    }
}
