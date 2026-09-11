use super::Sonarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join, set_api_path};

const BP_MEDIA_COVER: &str = "v3/mediacover";

impl Sonarr {
    /// Downloads a series media cover image (jpg, png, or gif).
    pub async fn get_media_cover(&self, series_id: i64, filename: &str) -> Result<Vec<u8>> {
        let encoded = percent_encode(filename);

        let uri = set_api_path(&path_join(&[
            BP_MEDIA_COVER,
            &str_val(series_id),
            &encoded,
        ]));

        let resp = self.api.get(Request::new(uri.clone())).await?;

        let body = resp.bytes().await.map_err(|source| Error::Http {
            context: format!("reading response body from {uri}"),
            source,
        })?;

        Ok(body.to_vec())
    }
}

/// Percent-encodes a single URL path segment, like Go's `url.PathEscape`.
fn percent_encode(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());

    for byte in segment.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'$' | b'&'
            | b'+' | b',' | b':' | b';' | b'=' | b'@' => out.push(byte as char),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }

    out
}
