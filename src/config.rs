//! The Starr app configuration and HTTP plumbing, ported from `starr.go` and `http.go`.

use crate::debuglog;
use crate::error::{Error, Result};
use crate::helpers;
use crate::interface::{ApiClient, InitializeJs, read_initialize_js};
use crate::req::{Request, parse_non_200, set_api_path};
use crate::values::Values;
use async_trait::async_trait;
use base64::prelude::{BASE64_STANDARD, Engine as _};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Method, Response, StatusCode};
use reqwest_cookie_store::CookieStoreMutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Default request timeout used by [`Config::new`].
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// The `User-Agent` this crate sends.
///
/// Kept byte-identical to the Go library so Starr apps and proxies see the
/// same client string after the migration.
pub const USER_AGENT: &str = "go-starr: https://golift.io/starr";

/// Config is the data needed to poll Radarr or Sonarr or Lidarr or Readarr.
///
/// At a minimum, provide a URL and API Key. `http_user` and `http_pass` are
/// used for Basic HTTP auth, if enabled (not common). `username` and
/// `password` are for non-API paths with native authentication enabled.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// The HTTP client used for every request.
    #[serde(skip)]
    pub client: Option<reqwest::Client>,
    /// The instance API key.
    #[serde(default, rename = "apiKey")]
    pub api_key: String,
    /// The instance base URL.
    #[serde(default)]
    pub url: String,
    /// Basic HTTP auth password.
    #[serde(default, rename = "httpPass")]
    pub http_pass: String,
    /// Basic HTTP auth user.
    #[serde(default, rename = "httpUser")]
    pub http_user: String,
    /// Native authentication username.
    #[serde(default)]
    pub username: String,
    /// Native authentication password.
    #[serde(default)]
    pub password: String,

    #[serde(skip)]
    cookie: AtomicBool,
    #[serde(skip)]
    cookie_store: Option<Arc<CookieStoreMutex>>,
    #[serde(skip)]
    debug: Option<Arc<debuglog::Config>>,
}

impl Config {
    /// Returns a Config that can plug into any Starr app.
    ///
    /// A zero `timeout` uses [`DEFAULT_TIMEOUT`]. Like the Go library, the
    /// client this builds does **not** verify TLS certificates; call
    /// [`Config::set_client`] with your own client to change that.
    pub fn new(api_key: impl Into<String>, app_url: impl Into<String>, timeout: Duration) -> Self {
        let timeout = if timeout.is_zero() {
            DEFAULT_TIMEOUT
        } else {
            timeout
        };

        let store = Arc::new(CookieStoreMutex::default());
        let client = helpers::build_client(timeout, false, Some(store.clone()))
            .expect("building the default starr HTTP client");

        Self {
            client: Some(client),
            api_key: api_key.into(),
            url: app_url.into(),
            cookie_store: Some(store),
            ..Default::default()
        }
    }

    /// Sets the Basic HTTP auth credentials (not common).
    pub fn with_http_auth(mut self, user: impl Into<String>, pass: impl Into<String>) -> Self {
        self.http_user = user.into();
        self.http_pass = pass.into();
        self
    }

    /// Sets the native authentication credentials used by [`ApiClient::login`].
    pub fn with_credentials(mut self, user: impl Into<String>, pass: impl Into<String>) -> Self {
        self.username = user.into();
        self.password = pass.into();
        self
    }

    /// Replaces the HTTP client.
    ///
    /// Cookie inspection during [`ApiClient::login`] is only available when the
    /// client was built by [`Config::new`], or when you also pass the cookie
    /// store via [`Config::set_cookie_store`].
    pub fn set_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self.cookie_store = None;
        self
    }

    /// Replaces the HTTP client and enables debug logging for this config.
    pub fn set_client_with_debug(
        mut self,
        client: reqwest::Client,
        debug: debuglog::Config,
    ) -> Self {
        self.client = Some(client);
        self.cookie_store = None;
        self.debug = Some(Arc::new(debug));
        self
    }

    /// Enables debug logging without replacing the client.
    pub fn with_debug(mut self, debug: debuglog::Config) -> Self {
        self.debug = Some(Arc::new(debug));
        self
    }

    /// Supplies the cookie store backing a custom client.
    pub fn set_cookie_store(mut self, store: Arc<CookieStoreMutex>) -> Self {
        self.cookie_store = Some(store);
        self
    }

    /// Reports whether [`ApiClient::login`] has stored an authentication cookie.
    pub fn has_cookie(&self) -> bool {
        self.cookie.load(Ordering::Relaxed)
    }

    /// Fills in a default client and trims the trailing slash from the URL.
    ///
    /// Every app module calls this from its `new()` constructor, matching the
    /// Go `New()` functions.
    pub fn prepare(mut self) -> Self {
        if self.client.is_none() {
            let store = Arc::new(CookieStoreMutex::default());
            self.client = Some(
                helpers::build_client(Duration::ZERO, false, Some(store.clone()))
                    .expect("building the default starr HTTP client"),
            );
            self.cookie_store = Some(store);
        }

        if let Some(trimmed) = self.url.strip_suffix('/') {
            self.url = trimmed.to_string();
        }

        self
    }

    /// Sets default request headers, merges `extra` (which overrides defaults),
    /// then forces `application/x-www-form-urlencoded` for native login POSTs.
    pub fn set_headers(
        &self,
        method: &Method,
        uri: &str,
        has_body: bool,
        extra: Option<&HeaderMap>,
    ) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        // This app allows http auth, in addition to api key (nginx proxy).
        let auth = format!("{}:{}", self.http_user, self.http_pass);
        if auth != ":" {
            let encoded = format!("Basic {}", BASE64_STANDARD.encode(auth.as_bytes()));
            headers.insert(AUTHORIZATION, header_value(&encoded)?);
        }

        if has_body {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        if let Some(extra) = extra {
            for (name, value) in extra {
                headers.insert(name.clone(), value.clone());
            }
        }

        if method == Method::POST && uri.ends_with("/login") {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        } else if !headers.contains_key(ACCEPT) {
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        }

        headers.insert(
            HeaderName::from_static("user-agent"),
            HeaderValue::from_static(USER_AGENT),
        );
        headers.insert(HeaderName::from_static("x-api-key"), header_value(&self.api_key)?);

        Ok(headers)
    }

    /// Our abstraction method for calling a Starr application.
    async fn do_req(&self, method: Method, req: Request) -> Result<Response> {
        let client = self.client.as_ref().ok_or(Error::NilClient)?;

        // Go trims exactly one trailing slash before appending the path.
        let base = self.url.strip_suffix('/').unwrap_or(&self.url);
        let mut url =
            reqwest::Url::parse(&format!("{base}{}", req.uri)).map_err(|source| Error::Url {
                context: format!("http.NewRequest({})", req.uri),
                source,
            })?;

        if let Some(query) = &req.query {
            let encoded = query.encode();
            url.set_query(if encoded.is_empty() {
                None
            } else {
                Some(&encoded)
            });
        }

        let headers =
            self.set_headers(&method, &req.uri, req.body.is_some(), req.headers.as_ref())?;

        let mut builder = client.request(method.clone(), url.clone()).headers(headers);

        let sent = req.body.unwrap_or_default();
        if !sent.is_empty() {
            builder = builder.body(sent.clone());
        }

        let started = Instant::now();

        let resp = match builder.send().await {
            Ok(resp) => resp,
            Err(source) => {
                if let Some(debug) = &self.debug {
                    debug.log_failure(method.as_str(), &sent, &source.to_string());
                }

                return Err(Error::Http {
                    context: "httpClient.Do(req)".to_string(),
                    source,
                });
            }
        };

        let resp = match &self.debug {
            Some(debug) => {
                log_and_rebuild(debug, &method, url.as_str(), resp, &sent, started.elapsed()).await?
            }
            None => resp,
        };

        let status = resp.status();
        if status < StatusCode::OK || status >= StatusCode::MULTIPLE_CHOICES {
            return Err(non_200_error(resp).await);
        }

        Ok(resp)
    }

    /// Counts the cookies the jar holds for our URL, when the jar is inspectable.
    fn jar_cookie_count(&self) -> Option<usize> {
        let store = self.cookie_store.as_ref()?;
        let url = reqwest::Url::parse(&self.url).ok()?;
        let store = store.lock().ok()?;

        Some(store.get_request_values(&url).count())
    }
}

#[async_trait]
impl ApiClient for Config {
    /// POSTs to the login form in a Starr app and saves the authentication
    /// cookie for future use.
    async fn login(&self) -> Result<()> {
        let mut params = Values::new();
        params.set("username", self.username.clone());
        params.set("password", self.password.clone());

        let req = Request::new("/login").with_body(params.encode());
        let mut code_err = crate::error::ReqError::default();

        match self.do_req(Method::POST, req).await {
            Ok(resp) => {
                // Protect against a nil map in case we don't get an error
                // (which should be impossible: login always redirects).
                code_err.headers = resp.headers().clone();
            }
            Err(err) => match err.as_req_error() {
                Some(req_err) => code_err = req_err.clone(),
                None => {
                    return Err(err.context(format!(
                        "invalid reply authenticating as user '{}'",
                        self.username
                    )));
                }
            },
        }

        let jar_is_empty = self.jar_cookie_count().is_some_and(|count| count == 0);

        if code_err.get("Location").contains("loginFailed") || jar_is_empty {
            return Err(Error::Request(format!(
                "authenticating as user '{}' failed",
                self.username
            )));
        }

        self.cookie.store(true, Ordering::Relaxed);

        Ok(())
    }

    /// Returns the data from the `initialize.js` file.
    ///
    /// If the instance requires authentication, you must call
    /// [`ApiClient::login`] before this method.
    ///
    /// Note: the URL is concatenated without a separator, exactly as the Go
    /// library does, so `url` must end with `/` for this call to resolve.
    async fn get_initialize_js(&self) -> Result<InitializeJs> {
        let client = self.client.as_ref().ok_or(Error::NilClient)?;
        let url = format!("{}initialize.js", self.url);

        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|source| Error::Http {
                context: "httpClient.Do(req)".to_string(),
                source,
            })?;

        if resp.status() != StatusCode::OK {
            return Err(Error::Req(Box::new(crate::error::ReqError {
                code: resp.status().as_u16() as i32,
                headers: resp.headers().clone(),
                ..Default::default()
            })));
        }

        let body = resp.text().await.map_err(|source| Error::Http {
            context: "scanning HTTP response".to_string(),
            source,
        })?;

        Ok(read_initialize_js(&body))
    }

    async fn req(&self, method: Method, req: Request) -> Result<Response> {
        self.do_req(method, req).await
    }

    async fn api(&self, method: Method, mut req: Request) -> Result<Response> {
        req.uri = set_api_path(&req.uri);
        self.do_req(method, req).await
    }
}

/// Builds a [`Error::Req`] from a non-2xx response.
async fn non_200_error(resp: Response) -> Error {
    let code = resp.status().as_u16() as i32;
    let headers = resp.headers().clone();

    match resp.bytes().await {
        Ok(body) => Error::Req(Box::new(parse_non_200(code, headers, body.to_vec()))),
        Err(source) => Error::Req(Box::new(crate::error::ReqError {
            headers,
            code,
            err: Some(source.to_string()),
            ..Default::default()
        })),
    }
}

/// Logs a request/response pair, then rebuilds the response so the caller can
/// still read the body.
async fn log_and_rebuild(
    debug: &debuglog::Config,
    method: &Method,
    url: &str,
    resp: Response,
    sent: &[u8],
    elapsed: Duration,
) -> Result<Response> {
    let status = resp.status();
    let headers = resp.headers().clone();
    let version = resp.version();

    let body = resp.bytes().await.map_err(|source| Error::Http {
        context: "reading Starr response body".to_string(),
        source,
    })?;

    debug.log_exchange(crate::debuglog::Exchange {
        method: method.as_str(),
        url,
        status: &format!("{} {}", status.as_u16(), status.canonical_reason().unwrap_or("")),
        headers: &headers,
        sent,
        rcvd: &body,
        elapsed,
    });

    let mut builder = ::http::Response::builder().status(status).version(version);

    if let Some(dest) = builder.headers_mut() {
        *dest = headers;
    }

    let rebuilt = builder
        .body(reqwest::Body::from(body))
        .map_err(|err| Error::Other(format!("rebuilding logged response: {err}")))?;

    Ok(Response::from(rebuilt))
}

fn header_value(value: &str) -> Result<HeaderValue> {
    HeaderValue::from_str(value)
        .map_err(|err| Error::Other(format!("invalid header value: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_match_go_defaults() {
        let config = Config::new("apikey123", "http://localhost:8989", Duration::ZERO);
        let headers = config
            .set_headers(&Method::GET, "/api/v3/system/status", false, None)
            .unwrap();

        assert_eq!(headers.get("x-api-key").unwrap(), "apikey123");
        assert_eq!(headers.get(ACCEPT).unwrap(), "application/json");
        assert_eq!(headers.get(USER_AGENT_HEADER).unwrap(), USER_AGENT);
        assert!(headers.get(CONTENT_TYPE).is_none());
        assert!(headers.get(AUTHORIZATION).is_none());
    }

    const USER_AGENT_HEADER: &str = "user-agent";

    #[test]
    fn body_sets_json_content_type() {
        let config = Config::new("k", "http://localhost", Duration::ZERO);
        let headers = config
            .set_headers(&Method::POST, "/api/v3/tag", true, None)
            .unwrap();

        assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "application/json");
    }

    #[test]
    fn login_post_forces_form_content_type() {
        let config = Config::new("k", "http://localhost", Duration::ZERO);
        let headers = config
            .set_headers(&Method::POST, "/login", true, None)
            .unwrap();

        assert_eq!(
            headers.get(CONTENT_TYPE).unwrap(),
            "application/x-www-form-urlencoded"
        );
        // Go does not set Accept on the login POST.
        assert!(headers.get(ACCEPT).is_none());
    }

    #[test]
    fn http_auth_sets_basic_header() {
        let config = Config::new("k", "http://localhost", Duration::ZERO).with_http_auth("me", "pw");
        let headers = config
            .set_headers(&Method::GET, "/api/v3/tag", false, None)
            .unwrap();

        assert_eq!(headers.get(AUTHORIZATION).unwrap(), "Basic bWU6cHc=");
    }

    #[test]
    fn extra_headers_override_defaults() {
        let config = Config::new("k", "http://localhost", Duration::ZERO);
        let mut extra = HeaderMap::new();
        extra.insert(ACCEPT, HeaderValue::from_static("text/plain"));

        let headers = config
            .set_headers(&Method::GET, "/api/v3/tag", false, Some(&extra))
            .unwrap();

        assert_eq!(headers.get(ACCEPT).unwrap(), "text/plain");
    }

    #[test]
    fn prepare_trims_one_trailing_slash() {
        let config = Config::new("k", "http://localhost:8686/", Duration::ZERO).prepare();
        assert_eq!(config.url, "http://localhost:8686");
    }
}
