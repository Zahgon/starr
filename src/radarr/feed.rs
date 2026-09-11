use super::Radarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::string_enum;
use crate::values::Values;
use serde::{Deserialize, Serialize};

/// This is not an `/api` path.
const BP_FEED: &str = "/feed/v3/calendar/radarr.ics";

string_enum! {
    /// ReleaseType is the type of release, found in a calendar feed.
    pub struct ReleaseType {
        /// The cinema release date.
        const CINEMA = "cinemaRelease";
        /// The digital release date.
        const DIGITAL = "digitalRelease";
        /// The physical media release date.
        const PHYSICAL = "physicalRelease";
    }
}

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
    /// Include unmonitored movies in the iCal feed.
    #[serde(default)]
    pub unmonitored: bool,
    /// Release dates to include in the feed.
    #[serde(default)]
    pub release_types: Vec<ReleaseType>,
    /// Events will appear as all day events in your calendar.
    #[serde(default)]
    pub as_all_day: bool,
}

impl Radarr {
    /// Returns the Calendar ICS feed file.
    pub async fn get_feed(&self, filter: &Feed) -> Result<Vec<u8>> {
        let tags: Vec<String> = filter.tags.iter().map(|tag| str_val(*tag)).collect();
        let release_types: Vec<&str> = filter
            .release_types
            .iter()
            .map(|release_type| release_type.as_str())
            .collect();

        let mut query = Values::new();
        query.set("unmonitored", str_val(filter.unmonitored));
        query.set("pastDays", str_val(filter.past_days));
        query.set("futureDays", str_val(filter.future_days));
        query.set("tags", tags.join(","));
        query.set("asAllDay", str_val(filter.as_all_day));
        query.set("releaseTypes", release_types.join(","));

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
