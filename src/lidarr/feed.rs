use super::Lidarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::values::Values;
use serde::{Deserialize, Serialize};

/// This is not an `/api` path.
///
/// The Go constant omits the leading slash, which produces a malformed URL
/// when joined to the app address; this port adds it so the endpoint works.
const BP_FEED: &str = "/feed/v1/calendar/lidarr.ics";

/// Feed is the `/feed/v1/calendar` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    /// Default Value: 7.
    #[serde(default)]
    pub past_days: i32,
    /// Default Value: 28.
    #[serde(default)]
    pub future_days: i32,
    /// Only include albums with these tags.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Include unmonitored albums in the iCal feed.
    #[serde(default)]
    pub unmonitored: bool,
}

impl Lidarr {
    /// Returns the Calendar ICS feed file.
    pub async fn get_feed(&self, filter: &Feed) -> Result<Vec<u8>> {
        let tags: Vec<String> = filter.tags.iter().map(|tag| str_val(*tag)).collect();

        let mut query = Values::new();
        query.set("unmonitored", str_val(filter.unmonitored));
        query.set("pastDays", str_val(filter.past_days));
        query.set("futureDays", str_val(filter.future_days));
        query.set("tags", tags.join(","));

        let resp = self
            .api
            .get(Request::new(BP_FEED).with_query(query))
            .await?;

        let body = resp.bytes().await.map_err(|source| Error::Http {
            context: "io.ReadAll".to_string(),
            source,
        })?;

        Ok(body.to_vec())
    }
}
