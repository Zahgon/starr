//! Shared structs and constants for all the Starr apps, ported from `shared.go`.

use crate::helpers::str_val;
use crate::is_default;
use crate::string_enum;
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::time::Duration;

/// The chrono time format the calendar expects the filter to be in.
///
/// Ported verbatim from Go's `2006-01-02T03:04:05.000Z`; note that `03`/`%I`
/// is a 12-hour clock in both languages.
pub const CALENDAR_TIME_FILTER_FORMAT: &str = "%Y-%m-%dT%I:%M:%S%.3fZ";

/// StatusMessage represents the status of the item. All apps use this.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StatusMessage {
    /// Title of the status message.
    #[serde(default)]
    pub title: String,
    /// The messages themselves.
    #[serde(default)]
    pub messages: Vec<String>,
}

/// BaseQuality is a base quality profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BaseQuality {
    /// Quality ID.
    #[serde(default)]
    pub id: i64,
    /// Quality name.
    #[serde(default)]
    pub name: String,
    /// Source, eg. `bluray`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub source: String,
    /// Vertical resolution, eg. `1080`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub resolution: i32,
    /// Modifier, eg. `remux`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub modifier: String,
}

/// QualityRevision is probably used in Sonarr.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QualityRevision {
    /// Revision version.
    #[serde(default)]
    pub version: i64,
    /// Real revision.
    #[serde(default)]
    pub real: i64,
    /// Whether this is a repack.
    #[serde(default, rename = "isRepack", skip_serializing_if = "is_default")]
    pub is_repack: bool,
}

/// Quality is a download quality profile attached to a movie, book, track or series.
///
/// It may contain 1 or more profiles. Neither Sonarr nor Readarr use
/// `name` or `id` in this struct.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Quality {
    /// Profile name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Profile ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// The base quality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<BaseQuality>,
    /// Nested quality items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Quality>>,
    /// Whether this quality is allowed.
    #[serde(default)]
    pub allowed: bool,
    /// Not sure which app had this....
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<QualityRevision>,
}

/// Ratings belong to a few types.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Ratings {
    /// Number of votes.
    #[serde(default)]
    pub votes: i64,
    /// Rating value.
    #[serde(default)]
    pub value: f64,
    /// Popularity score.
    #[serde(default, skip_serializing_if = "is_default")]
    pub popularity: f64,
    /// Rating type.
    #[serde(default, rename = "type", skip_serializing_if = "is_default")]
    pub rating_type: String,
}

/// OpenRatings is a ratings type that has a source and type.
pub type OpenRatings = HashMap<String, Ratings>;

/// IsLoaded is a generic struct used in a few places.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IsLoaded {
    /// Whether the resource is loaded.
    #[serde(default, rename = "isLoaded")]
    pub is_loaded: bool,
}

/// Link is used in a few places.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// Link URL.
    #[serde(default)]
    pub url: String,
    /// Link name.
    #[serde(default)]
    pub name: String,
}

/// Tag may be applied to nearly anything.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// Tag ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Tag label.
    #[serde(default)]
    pub label: String,
}

/// Image is used in a few places.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Image {
    /// Cover type, eg. `poster`.
    #[serde(default, rename = "coverType")]
    pub cover_type: String,
    /// Local URL.
    #[serde(default, skip_serializing_if = "is_default")]
    pub url: String,
    /// Remote URL.
    #[serde(default, rename = "remoteUrl", skip_serializing_if = "is_default")]
    pub remote_url: String,
    /// File extension.
    #[serde(default, skip_serializing_if = "is_default")]
    pub extension: String,
}

/// Path is for unmanaged folder paths.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Path {
    /// Folder name.
    #[serde(default)]
    pub name: String,
    /// Folder path.
    #[serde(default)]
    pub path: String,
}

/// RemotePathMapping is the remotePathMapping endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemotePathMapping {
    /// Mapping ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Download client host.
    #[serde(default)]
    pub host: String,
    /// Path as the download client sees it.
    #[serde(default, rename = "remotePath")]
    pub remote_path: String,
    /// Path as the Starr app sees it.
    #[serde(default, rename = "localPath")]
    pub local_path: String,
}

/// Value is a generic ID/Name struct applied to a few places.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Value {
    /// Value ID.
    #[serde(default)]
    pub id: i64,
    /// Value name.
    #[serde(default)]
    pub name: String,
}

/// SelectOption is part of Field.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    /// Whether a divider follows this option.
    #[serde(default, rename = "dividerAfter", skip_serializing_if = "is_default")]
    pub divider_after: bool,
    /// Display order.
    #[serde(default)]
    pub order: i64,
    /// Option value.
    #[serde(default)]
    pub value: i64,
    /// Option hint.
    #[serde(default)]
    pub hint: String,
    /// Option name.
    #[serde(default)]
    pub name: String,
}

/// FieldOutput is a generic Name/Value struct applied to a few places.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldOutput {
    /// Whether the field is an advanced setting.
    #[serde(default, skip_serializing_if = "is_default")]
    pub advanced: bool,
    /// Display order.
    #[serde(default, skip_serializing_if = "is_default")]
    pub order: i64,
    /// Link to further help.
    #[serde(default, rename = "helpLink", skip_serializing_if = "is_default")]
    pub help_link: String,
    /// Help text.
    #[serde(default, rename = "helpText", skip_serializing_if = "is_default")]
    pub help_text: String,
    /// Hidden state; a string in the API, not a bool.
    #[serde(default, skip_serializing_if = "is_default")]
    pub hidden: String,
    /// Field label.
    #[serde(default, skip_serializing_if = "is_default")]
    pub label: String,
    /// Field name.
    #[serde(default)]
    pub name: String,
    /// Provider action used to populate select options.
    #[serde(
        default,
        rename = "selectOptionsProviderAction",
        skip_serializing_if = "is_default"
    )]
    pub select_options_provider_action: String,
    /// Field type.
    #[serde(default, rename = "type", skip_serializing_if = "is_default")]
    pub field_type: String,
    /// Privacy setting.
    #[serde(default)]
    pub privacy: String,
    /// Field value; may be any JSON type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// Available select options.
    #[serde(
        default,
        rename = "selectOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub select_options: Option<Vec<SelectOption>>,
}

/// FieldInput is a generic Name/Value struct applied to a few places.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldInput {
    /// Field name.
    #[serde(default)]
    pub name: String,
    /// Field value; may be any JSON type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// KeyValue is yet another reusable generic type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    /// The key.
    #[serde(default)]
    pub key: String,
    /// The value.
    #[serde(default)]
    pub value: i32,
}

/// BackupFile comes from the system/backup paths in all apps.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackupFile {
    /// File name.
    #[serde(default)]
    pub name: String,
    /// File path, used with a raw GET to download the backup.
    #[serde(default)]
    pub path: String,
    /// Backup type, eg. `scheduled`.
    #[serde(default, rename = "type")]
    pub backup_type: String,
    /// When the backup was taken.
    #[serde(default)]
    pub time: Option<DateTime<Utc>>,
    /// Backup ID.
    #[serde(default)]
    pub id: i64,
    /// Size in bytes.
    #[serde(default)]
    pub size: i64,
}

/// QueueDeleteOpts are the extra inputs when deleting an item from the Activity Queue.
///
/// Set these appropriately for your expectations. All inputs are the same in
/// all apps. Passing `None` to the queue-delete methods uses the defaults
/// shown below.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct QueueDeleteOpts {
    /// Default true; use `Some(false)` to change it.
    pub remove_from_client: Option<bool>,
    /// Default false.
    pub block_list: bool,
    /// Default false.
    pub skip_redownload: bool,
    /// Default false.
    pub change_category: bool,
}

impl QueueDeleteOpts {
    /// Turns delete options into HTTP GET query parameters.
    ///
    /// Call on `Option<&QueueDeleteOpts>` via [`queue_delete_values`] when the
    /// caller did not supply options at all.
    pub fn values(&self) -> Values {
        let mut params = Values::new();
        params.set("removeFromClient", "true");
        params.set("blocklist", str_val(self.block_list));
        params.set("skipRedownload", str_val(self.skip_redownload));
        params.set("changeCategory", str_val(self.change_category));

        if let Some(remove) = self.remove_from_client {
            params.set("removeFromClient", str_val(remove));
        }

        params
    }
}

/// Turns optional delete options into query parameters.
///
/// `None` yields only `removeFromClient=true`, matching the Go nil receiver.
pub fn queue_delete_values(opts: Option<&QueueDeleteOpts>) -> Values {
    match opts {
        Some(opts) => opts.values(),
        None => {
            let mut params = Values::new();
            params.set("removeFromClient", "true");
            params
        }
    }
}

/// FormatItem is part of a quality profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FormatItem {
    /// Custom format ID.
    #[serde(default)]
    pub format: i64,
    /// Custom format name.
    #[serde(default)]
    pub name: String,
    /// Custom format score.
    #[serde(default)]
    pub score: i64,
}

/// TimeSpan is used when a Starr API returns a duration as an object.
///
/// For `ParsedTrackInfo`/`audioTags` the APIs use a string with format
/// `date-span` instead; use a string there.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSpan {
    /// Duration in 100ns ticks.
    #[serde(default)]
    pub ticks: i64,
    /// Whole days component.
    #[serde(default)]
    pub days: i64,
    /// Whole hours component.
    #[serde(default)]
    pub hours: i64,
    /// Whole milliseconds component.
    #[serde(default)]
    pub milliseconds: i64,
    /// Whole minutes component.
    #[serde(default)]
    pub minutes: i64,
    /// Whole seconds component.
    #[serde(default)]
    pub seconds: i64,
    /// Total duration in days.
    #[serde(default)]
    pub total_days: i64,
    /// Total duration in hours.
    #[serde(default)]
    pub total_hours: i64,
    /// Total duration in milliseconds.
    #[serde(default)]
    pub total_milliseconds: i64,
    /// Total duration in minutes.
    #[serde(default)]
    pub total_minutes: i64,
    /// Total duration in seconds.
    #[serde(default)]
    pub total_seconds: i64,
}

string_enum! {
    /// ApplyTags is an enum used as an input for bulk editors, and perhaps other places.
    ///
    /// Schema documented at
    /// <https://radarr.video/docs/api/#/MovieEditor/put_api_v3_movie_editor>.
    pub struct ApplyTags {
        /// Add the given tags.
        const ADD = "add";
        /// Remove the given tags.
        const REMOVE = "remove";
        /// Replace all tags with the given tags.
        const REPLACE = "replace";
    }
}

string_enum! {
    /// Protocol used to download media.
    pub struct Protocol {
        /// Unknown protocol.
        const UNKNOWN = "unknown";
        /// Usenet.
        const USENET = "usenet";
        /// BitTorrent.
        const TORRENT = "torrent";
    }
}

/// BulkIndexer is the input to `update_indexers` on all apps except Prowlarr.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BulkIndexer {
    /// Indexer IDs to edit.
    #[serde(default)]
    pub ids: Vec<i64>,
    /// Tag IDs to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// How to apply the tags.
    #[serde(default, rename = "applyTags", skip_serializing_if = "is_default")]
    pub apply_tags: ApplyTags,
    /// Toggle RSS sync.
    #[serde(default, rename = "enableRss", skip_serializing_if = "Option::is_none")]
    pub enable_rss: Option<bool>,
    /// Toggle automatic search.
    #[serde(
        default,
        rename = "enableAutomaticSearch",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_automatic_search: Option<bool>,
    /// Toggle interactive search.
    #[serde(
        default,
        rename = "enableInteractiveSearch",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_interactive_search: Option<bool>,
    /// Indexer priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

/// PlayTime is used in at least Sonarr, maybe other places.
///
/// Holds a duration converted from `hh:mm:ss`, keeping the original string so
/// it can be echoed back to the API unchanged.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlayTime {
    /// The parsed duration.
    pub duration: Duration,
    /// The original string received from the API.
    pub original: String,
}

impl PlayTime {
    /// Parses a run time duration in format `hh:mm:ss` or `hh:mm:ss.fraction`.
    pub fn parse(original: impl Into<String>) -> Self {
        let original = original.into();
        let parts: Vec<&str> = original.split(':').collect();

        // Go's strconv ignores parse failures here and leaves a zero component.
        let seconds = |val: &str| val.parse::<f64>().unwrap_or(0.0);
        let whole = |val: &str| val.parse::<u64>().unwrap_or(0);

        let secs = match parts.len() {
            // hh:mm:ss or hh:mm:ss.fraction
            3 => whole(parts[0]) as f64 * 3600.0 + whole(parts[1]) as f64 * 60.0 + seconds(parts[2]),
            // mm:ss or mm:ss.fraction
            2 => whole(parts[0]) as f64 * 60.0 + seconds(parts[1]),
            // ss or ss.fraction
            1 => seconds(parts[0]),
            _ => 0.0,
        };

        Self {
            duration: Duration::from_secs_f64(secs.max(0.0)),
            original,
        }
    }

    /// Total number of seconds in this play time.
    pub fn seconds(&self) -> f64 {
        self.duration.as_secs_f64()
    }
}

impl<'de> Deserialize<'de> for PlayTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Go trims quotes off the raw JSON token, so numbers and null arrive
        // as their literal text. Reproduce that behavior here.
        let value = serde_json::Value::deserialize(deserializer)?;

        let original = match value {
            serde_json::Value::String(text) => text,
            other => other.to_string(),
        };

        Ok(PlayTime::parse(original))
    }
}

impl Serialize for PlayTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if !self.original.is_empty() {
            return serializer.serialize_str(&self.original);
        }

        // Format the duration as hh:mm:ss(.fraction) to match the API shape.
        let total = self.seconds();
        if total == 0.0 {
            return serializer.serialize_str("00:00:00");
        }

        let hours = (total / 3600.0) as i64;
        let mins = ((total - (hours * 3600) as f64) / 60.0) as i64;
        let secs = total - (hours * 3600) as f64 - (mins * 60) as f64;

        // Go uses %02d for hours and minutes but plain FormatFloat for seconds.
        serializer.serialize_str(&format!("{hours:02}:{mins:02}:{}", str_val(secs)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playtime_parses_all_shapes() {
        assert_eq!(PlayTime::parse("01:30:00").duration.as_secs(), 5400);
        assert_eq!(PlayTime::parse("30:00").duration.as_secs(), 1800);
        assert_eq!(PlayTime::parse("45").duration.as_secs(), 45);
        assert_eq!(PlayTime::parse("00:00:01.5").duration.as_millis(), 1500);
    }

    #[test]
    fn playtime_roundtrips_original_string() {
        let parsed: PlayTime = serde_json::from_str(r#""01:30:00""#).unwrap();
        assert_eq!(parsed.duration.as_secs(), 5400);
        assert_eq!(serde_json::to_string(&parsed).unwrap(), r#""01:30:00""#);
    }

    #[test]
    fn playtime_formats_when_original_is_empty() {
        let play = PlayTime {
            duration: Duration::from_secs(3723),
            original: String::new(),
        };
        assert_eq!(serde_json::to_string(&play).unwrap(), r#""01:02:3""#);

        let zero = PlayTime::default();
        assert_eq!(serde_json::to_string(&zero).unwrap(), r#""00:00:00""#);
    }

    #[test]
    fn queue_delete_opts_default_values() {
        assert_eq!(
            queue_delete_values(None).encode(),
            "removeFromClient=true"
        );

        let opts = QueueDeleteOpts {
            remove_from_client: Some(false),
            block_list: true,
            ..Default::default()
        };
        assert_eq!(
            opts.values().encode(),
            "blocklist=true&changeCategory=false&removeFromClient=false&skipRedownload=false"
        );
    }

    #[test]
    fn omitempty_fields_are_skipped() {
        let tag = Tag {
            id: 0,
            label: "hi".into(),
        };
        assert_eq!(serde_json::to_string(&tag).unwrap(), r#"{"label":"hi"}"#);
    }
}
