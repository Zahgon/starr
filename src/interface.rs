//! The client interface used by the app modules, ported from `interface.go`.

use crate::error::{Error, ErrorContext, Result};
use crate::req::Request;
use async_trait::async_trait;
use reqwest::{Method, Response};
use serde::de::DeserializeOwned;

/// ApiClient is used by the app modules to allow mocking the HTTP methods in tests.
///
/// It changes once in a while, so avoid making hard dependencies on it.
/// The generic `*_into` helpers live on [`ApiClientExt`] so this trait stays
/// object safe and can be stored as `Arc<dyn ApiClient>`.
#[async_trait]
pub trait ApiClient: std::fmt::Debug + Send + Sync {
    /// Used for non-API paths, like downloading backups or the initialize.js file.
    async fn login(&self) -> Result<()>;

    /// Returns the data from the `initialize.js` file.
    async fn get_initialize_js(&self) -> Result<InitializeJs>;

    /// Makes an authenticated request to a Starr application and returns the response.
    ///
    /// This does not ensure an `/api` URI prefix; it is for non-api paths.
    async fn req(&self, method: Method, req: Request) -> Result<Response>;

    /// Calls an API path, ensuring the `/api` URI prefix.
    async fn api(&self, method: Method, req: Request) -> Result<Response>;
}

/// Convenience methods layered on top of [`ApiClient`].
///
/// Implemented for every `ApiClient`, including `dyn ApiClient`.
#[async_trait]
pub trait ApiClientExt: ApiClient {
    /// Makes a GET HTTP request and returns the response. Params are optional.
    ///
    /// Do not use this in app methods; it does not add the `/api` prefix.
    async fn get(&self, req: Request) -> Result<Response> {
        let uri = req.uri.clone();
        self.req(Method::GET, req)
            .await
            .ctx(|| format!("api.Get({uri})"))
    }

    /// Makes a POST HTTP request and returns the response.
    async fn post(&self, req: Request) -> Result<Response> {
        let uri = req.uri.clone();
        self.req(Method::POST, req)
            .await
            .ctx(|| format!("api.Post({uri})"))
    }

    /// Makes a PUT HTTP request and returns the response.
    async fn put(&self, req: Request) -> Result<Response> {
        let uri = req.uri.clone();
        self.req(Method::PUT, req)
            .await
            .ctx(|| format!("api.Put({uri})"))
    }

    /// Makes a DELETE HTTP request and returns the response.
    async fn delete(&self, req: Request) -> Result<Response> {
        let uri = req.uri.clone();
        self.req(Method::DELETE, req)
            .await
            .ctx(|| format!("api.Delete({uri})"))
    }

    /// Performs an HTTP GET against an API path and deserializes the payload.
    ///
    /// Errors are wrapped with `api.Get(<uri>)`, matching the message the Go
    /// library builds at each call site.
    async fn get_into<T: DeserializeOwned + Send>(&self, req: Request) -> Result<T> {
        let uri = req.uri.clone();
        decode(self.api(Method::GET, req).await)
            .await
            .ctx(|| format!("api.Get({uri})"))
    }

    /// Performs an HTTP POST against an API path and deserializes the payload.
    async fn post_into<T: DeserializeOwned + Send>(&self, req: Request) -> Result<T> {
        let uri = req.uri.clone();
        decode(self.api(Method::POST, req).await)
            .await
            .ctx(|| format!("api.Post({uri})"))
    }

    /// Performs an HTTP PUT against an API path and deserializes the payload.
    async fn put_into<T: DeserializeOwned + Send>(&self, req: Request) -> Result<T> {
        let uri = req.uri.clone();
        decode(self.api(Method::PUT, req).await)
            .await
            .ctx(|| format!("api.Put({uri})"))
    }

    /// Performs an HTTP DELETE against an API path; output is ignored.
    async fn delete_any(&self, req: Request) -> Result<()> {
        let uri = req.uri.clone();
        self.api(Method::DELETE, req)
            .await
            .ctx(|| format!("api.Delete({uri})"))?;
        Ok(())
    }

    /// Performs an HTTP POST against an API path; output is ignored.
    ///
    /// Go spells this `PostInto(ctx, req, &output)` with an `any` output that
    /// the caller discards; endpoints like `system/restart` use it.
    async fn post_any(&self, req: Request) -> Result<()> {
        let _: serde_json::Value = self.post_into(req).await?;
        Ok(())
    }

    /// Performs an HTTP PUT against an API path; output is ignored.
    async fn put_any(&self, req: Request) -> Result<()> {
        let _: serde_json::Value = self.put_into(req).await?;
        Ok(())
    }
}

impl<T: ApiClient + ?Sized> ApiClientExt for T {}

/// Checks an error and decodes the JSON response body.
async fn decode<T: DeserializeOwned>(resp: Result<Response>) -> Result<T> {
    let resp = resp?;

    let body = resp.bytes().await.map_err(|source| Error::Http {
        context: "reading Starr response body".to_string(),
        source,
    })?;

    serde_json::from_slice(&body).map_err(|source| Error::Json {
        context: "decoding Starr JSON response body".to_string(),
        source,
    })
}

/// InitializeJS is the data contained in the `initialize.js` file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InitializeJs {
    /// Application name, taken from the `window.Xxxx` line.
    pub app: String,
    /// API root path, eg. `/api/v3`.
    pub api_root: String,
    /// The instance API key.
    pub api_key: String,
    /// Release string.
    pub release: String,
    /// Version string.
    pub version: String,
    /// Configured instance name.
    pub instance_name: String,
    /// UI theme.
    pub theme: String,
    /// Release branch.
    pub branch: String,
    /// Analytics setting.
    pub analytics: String,
    /// User hash.
    pub user_hash: String,
    /// Configured URL base.
    pub url_base: String,
    /// Whether the app is a production build.
    pub is_production: bool,
}

/// Parses the contents of an `initialize.js` file.
pub fn read_initialize_js(input: &str) -> InitializeJs {
    let mut output = InitializeJs::default();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        if split.len() < 2 {
            continue;
        }

        let trim = |val: &str| val.trim_matches(['"', '\'', ',']).to_string();

        match split[0] {
            "apiRoot:" => output.api_root = trim(split[1]),
            "apiKey:" => output.api_key = trim(split[1]),
            "version:" => output.version = trim(split[1]),
            "release:" => output.release = trim(split[1]),
            "instanceName:" => output.instance_name = trim(split[1]),
            "theme:" => output.theme = trim(split[1]),
            "branch:" => output.branch = trim(split[1]),
            "analytics:" => output.analytics = trim(split[1]),
            "userHash:" => output.user_hash = trim(split[1]),
            "urlBase:" => output.url_base = trim(split[1]),
            "isProduction:" => output.is_production = trim(split[1]) == "true",
            other if other.starts_with("window.") => {
                output.app = other.trim_start_matches("window.").to_string();
            }
            _ => {}
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_initialize_js() {
        let input = r#"
window.Sonarr = {
  apiRoot: '/api/v3',
  apiKey: 'abc123',
  release: '4.0.0.111',
  version: '4.0.0.111',
  instanceName: 'Sonarr',
  theme: 'auto',
  branch: 'main',
  analytics: 'True',
  userHash: 'deadbeef',
  urlBase: '',
  isProduction: true
};
"#;

        let out = read_initialize_js(input);
        assert_eq!(out.app, "Sonarr");
        assert_eq!(out.api_root, "/api/v3");
        assert_eq!(out.api_key, "abc123");
        assert_eq!(out.instance_name, "Sonarr");
        assert!(out.is_production);
    }
}
