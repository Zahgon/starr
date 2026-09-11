use super::Sonarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::values::Values;
use serde::{Deserialize, Serialize};

/// This is not an `/api` path.
const BP_FEED: &str = "/feed/v3/calendar/sonarr.ics";

/// Feed is the `/feed/v3/calendar` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    /// Default value: 7.
    #[serde(default)]
    pub past_days: i32,
    /// Default value: 28.
    #[serde(default)]
    pub future_days: i32,
    /// Tags to filter the feed by.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Include unmonitored tv shows in the iCal feed.
    #[serde(default)]
    pub unmonitored: bool,
    /// Only include premieres in the iCal feed.
    #[serde(default)]
    pub premieres_only: bool,
    /// Events will appear as all day events in your calendar.
    #[serde(default)]
    pub as_all_day: bool,
}

impl Sonarr {
    /// Returns the Calendar ICS feed file.
    pub async fn get_feed(&self, filter: &Feed) -> Result<Vec<u8>> {
        let tags: Vec<String> = filter.tags.iter().map(|tag| str_val(*tag)).collect();

        let mut query = Values::new();
        query.set("unmonitored", str_val(filter.unmonitored));
        query.set("pastDays", str_val(filter.past_days));
        query.set("futureDays", str_val(filter.future_days));
        query.set("tags", tags.join(","));
        query.set("asAllDay", str_val(filter.as_all_day));
        query.set("premieresOnly", str_val(filter.premieres_only));

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
