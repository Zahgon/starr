use super::{Episode, Sonarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::shared::CALENDAR_TIME_FILTER_FORMAT;
use crate::values::Values;
use chrono::{DateTime, Utc};

const BP_CALENDAR: &str = "v3/calendar";

/// Calendar defines the filters for fetching calendar items.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Calendar {
    /// Earliest date to return.
    pub start: Option<DateTime<Utc>>,
    /// Latest date to return.
    pub end: Option<DateTime<Utc>>,
    /// Whether unmonitored episodes are included.
    pub unmonitored: bool,
    /// Whether the series is included in each item.
    pub include_series: bool,
    /// Whether the episode file is included in each item.
    pub include_episode_file: bool,
    /// Whether episode images are included in each item.
    pub include_episode_images: bool,
}

impl Sonarr {
    /// Returns calendars based on filters.
    pub async fn get_calendar(&self, filter: &Calendar) -> Result<Vec<Episode>> {
        let mut query = Values::new();
        query.add("unmonitored", str_val(filter.unmonitored));
        query.add("includeSeries", str_val(filter.include_series));
        query.add("includeEpisodeFile", str_val(filter.include_episode_file));
        query.add("includeEpisodeImages", str_val(filter.include_episode_images));

        if let Some(start) = filter.start {
            query.add(
                "start",
                start.format(CALENDAR_TIME_FILTER_FORMAT).to_string(),
            );
        }

        if let Some(end) = filter.end {
            query.add("end", end.format(CALENDAR_TIME_FILTER_FORMAT).to_string());
        }

        self.api
            .get_into(Request::new(BP_CALENDAR).with_query(query))
            .await
    }

    /// Returns a single calendar item by ID.
    pub async fn get_calendar_id(&self, calendar_id: i64) -> Result<Episode> {
        self.api
            .get_into(Request::new(crate::req::path_join(&[
                BP_CALENDAR,
                &str_val(calendar_id),
            ])))
            .await
    }
}
