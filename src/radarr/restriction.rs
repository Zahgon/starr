use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_RESTRICTION: &str = "v3/restriction";

/// Restriction is the input for a new or updated restriction.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    /// Tags this restriction applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Terms a release must contain.
    #[serde(default, skip_serializing_if = "is_default")]
    pub required: String,
    /// Terms a release must not contain.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ignored: String,
    /// Restriction ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
}

impl Radarr {
    /// Returns all configured restrictions.
    pub async fn get_restrictions(&self) -> Result<Vec<Restriction>> {
        self.api.get_into(Request::new(BP_RESTRICTION)).await
    }

    /// Returns a single restriction.
    pub async fn get_restriction(&self, restriction_id: i64) -> Result<Restriction> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_RESTRICTION,
                &str_val(restriction_id),
            ])))
            .await
    }

    /// Creates a restriction.
    pub async fn add_restriction(&self, restriction: &Restriction) -> Result<Restriction> {
        self.api
            .post_into(Request::new(BP_RESTRICTION).with_json(restriction)?)
            .await
    }

    /// Updates the restriction.
    pub async fn update_restriction(&self, restriction: &Restriction) -> Result<Restriction> {
        let req = Request::new(path_join(&[BP_RESTRICTION, &str_val(restriction.id)]))
            .with_json(restriction)?;
        self.api.put_into(req).await
    }

    /// Removes a single restriction.
    pub async fn delete_restriction(&self, restriction_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_RESTRICTION,
                &str_val(restriction_id),
            ])))
            .await
    }
}
