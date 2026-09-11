use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::Value;
use serde::{Deserialize, Serialize};

const BP_LANGUAGE_PROFILE: &str = "v3/languageProfile";

/// LanguageProfile is the `/api/v3/languageprofile` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageProfile {
    /// Whether releases are upgraded until the cutoff is met.
    #[serde(default)]
    pub upgrade_allowed: bool,
    /// Profile ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Profile name.
    #[serde(default)]
    pub name: String,
    /// Language that stops upgrades.
    #[serde(default)]
    pub cutoff: Option<Value>,
    /// Languages allowed by this profile.
    #[serde(default)]
    pub languages: Vec<Language>,
}

/// Language is part of [`LanguageProfile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// Whether this language is allowed.
    #[serde(default)]
    pub allowed: bool,
    /// The language.
    #[serde(default)]
    pub language: Option<Value>,
}

impl Sonarr {
    /// Returns all configured language profiles.
    pub async fn get_language_profiles(&self) -> Result<Vec<LanguageProfile>> {
        self.api.get_into(Request::new(BP_LANGUAGE_PROFILE)).await
    }

    /// Returns a single language profile.
    pub async fn get_language_profile(&self, profile_id: i64) -> Result<LanguageProfile> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_LANGUAGE_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }

    /// Creates a language profile.
    pub async fn add_language_profile(
        &self,
        profile: &LanguageProfile,
    ) -> Result<LanguageProfile> {
        self.api
            .post_into(Request::new(BP_LANGUAGE_PROFILE).with_json(profile)?)
            .await
    }

    /// Updates the language profile.
    pub async fn update_language_profile(
        &self,
        profile: &LanguageProfile,
    ) -> Result<LanguageProfile> {
        let req = Request::new(path_join(&[BP_LANGUAGE_PROFILE, &str_val(profile.id)]))
            .with_json(profile)?;
        self.api.put_into(req).await
    }

    /// Removes a single language profile.
    pub async fn delete_language_profile(&self, profile_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_LANGUAGE_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }
}
