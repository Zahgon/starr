use super::{CustomFormatOutput, Movie, Radarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::shared::Value;
use crate::values::Values;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BP_PARSE: &str = "v3/parse";

/// ParseOutput is returned from `GET /api/v3/parse`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseOutput {
    /// Custom formats matched by the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub custom_formats: Vec<CustomFormatOutput>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
    /// Languages found in the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub languages: Vec<Value>,
    /// Parse result ID.
    #[serde(default)]
    pub id: i64,
    /// The title that was parsed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// The movie the release belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub movie: Option<Movie>,
    /// Everything parsed out of the title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub parsed_movie_info: HashMap<String, serde_json::Value>,
}

impl Radarr {
    /// Resolves a release title into parsed movie metadata.
    pub async fn parse(&self, title: &str) -> Result<ParseOutput> {
        let mut query = Values::new();
        query.set("title", title);

        self.api
            .get_into(Request::new(BP_PARSE).with_query(query))
            .await
    }
}
