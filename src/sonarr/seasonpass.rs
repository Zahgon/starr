use super::Sonarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_SEASON_PASS: &str = "v3/seasonPass";

/// SeasonPass is the input payload for a seasonPass update.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonPass {
    /// Series to change.
    #[serde(default)]
    pub series: Vec<MonitoredSeries>,
    /// How the series get monitored.
    #[serde(default)]
    pub monitoring_options: Option<MonitoringOptions>,
}

/// MonitoringOptions is part of the [`SeasonPass`] payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringOptions {
    /// Valid values are: `all`, `future`, `missing`, `existing`, `firstSeason`,
    /// `latestSeason`, and `none`.
    #[serde(default)]
    pub monitor: String,
}

/// MonitoredSeries is part of the [`SeasonPass`] payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoredSeries {
    /// Series ID.
    #[serde(default)]
    pub id: i32,
    /// Whether the series is monitored.
    #[serde(default)]
    pub monitored: bool,
}

impl Sonarr {
    /// Allows monitoring many Series and episodes at once.
    pub async fn update_season_pass(&self, season_pass: &SeasonPass) -> Result<()> {
        let req = Request::new(BP_SEASON_PASS).with_json(season_pass)?;
        self.api.post_any(req).await
    }
}
