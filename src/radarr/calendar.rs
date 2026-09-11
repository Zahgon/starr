use super::{Movie, Radarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::shared::CALENDAR_TIME_FILTER_FORMAT;
use crate::values::Values;
use chrono::{DateTime, Utc};

const BP_CALENDAR: &str = "v3/calendar";

/// Calendar defines the filters for fetching calendar items.
///
/// Start and End are required.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Calendar {
    /// Earliest date to return.
    pub start: Option<DateTime<Utc>>,
    /// Latest date to return.
    pub end: Option<DateTime<Utc>>,
    /// Whether unmonitored movies are included.
    pub unmonitored: bool,
}

impl Radarr {
    /// Returns calendars based on filters.
    pub async fn get_calendar(&self, filter: &Calendar) -> Result<Vec<Movie>> {
        let mut query = Values::new();
        query.add("unmonitored", str_val(filter.unmonitored));

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
}
