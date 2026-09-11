use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use serde::{Deserialize, Serialize};

const BP_NOTIFICATION: &str = "v3/notification";

/// NotificationInput is the input for a new or updated notification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationInput {
    /// Notify when a release is grabbed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_grab: bool,
    /// Notify when a release is imported.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_download: bool,
    /// Notify when a release is upgraded.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_upgrade: bool,
    /// Notify when files are renamed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_rename: bool,
    /// Notify when a series is deleted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_series_delete: bool,
    /// Notify when an episode file is deleted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_episode_file_delete: bool,
    /// Notify when an episode file is deleted for an upgrade.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_episode_file_delete_for_upgrade: bool,
    /// Notify on health issues.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_health_issue: bool,
    /// Notify when the application updates.
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_application_update: bool,
    /// Whether health warnings are included.
    #[serde(default, skip_serializing_if = "is_default")]
    pub include_health_warnings: bool,
    /// Notification ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Notification name.
    #[serde(default)]
    pub name: String,
    /// Implementation type of the notification.
    #[serde(default)]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Tags applied to this notification.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldInput>,
}

/// NotificationOutput is the output from the notification methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationOutput {
    /// Notify when a release is grabbed.
    #[serde(default)]
    pub on_grab: bool,
    /// Notify when a release is imported.
    #[serde(default)]
    pub on_download: bool,
    /// Notify when a release is upgraded.
    #[serde(default)]
    pub on_upgrade: bool,
    /// Notify when files are renamed.
    #[serde(default)]
    pub on_rename: bool,
    /// Notify when a series is deleted.
    #[serde(default)]
    pub on_series_delete: bool,
    /// Notify when an episode file is deleted.
    #[serde(default)]
    pub on_episode_file_delete: bool,
    /// Notify when an episode file is deleted for an upgrade.
    #[serde(default)]
    pub on_episode_file_delete_for_upgrade: bool,
    /// Notify on health issues.
    #[serde(default)]
    pub on_health_issue: bool,
    /// Notify when the application updates.
    #[serde(default)]
    pub on_application_update: bool,
    /// Whether the implementation supports grab events.
    #[serde(default)]
    pub supports_on_grab: bool,
    /// Whether the implementation supports import events.
    #[serde(default)]
    pub supports_on_download: bool,
    /// Whether the implementation supports upgrade events.
    #[serde(default)]
    pub supports_on_upgrade: bool,
    /// Whether the implementation supports rename events.
    #[serde(default)]
    pub supports_on_rename: bool,
    /// Whether the implementation supports series delete events.
    #[serde(default)]
    pub supports_on_series_delete: bool,
    /// Whether the implementation supports episode file delete events.
    #[serde(default)]
    pub supports_on_episode_file_delete: bool,
    /// Whether the implementation supports episode file delete for upgrade events.
    #[serde(default)]
    pub supports_on_episode_file_delete_for_upgrade: bool,
    /// Whether the implementation supports health issue events.
    #[serde(default)]
    pub supports_on_health_issue: bool,
    /// Whether the implementation supports application update events.
    #[serde(default)]
    pub supports_on_application_update: bool,
    /// Whether health warnings are included.
    #[serde(default)]
    pub include_health_warnings: bool,
    /// Notification ID.
    #[serde(default)]
    pub id: i64,
    /// Notification name.
    #[serde(default)]
    pub name: String,
    /// Display name of the implementation.
    #[serde(default)]
    pub implementation_name: String,
    /// Implementation type of the notification.
    #[serde(default)]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Link to more information about the notification.
    #[serde(default)]
    pub info_link: String,
    /// Tags applied to this notification.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
}

impl Sonarr {
    /// Returns all configured notifications.
    pub async fn get_notifications(&self) -> Result<Vec<NotificationOutput>> {
        self.api.get_into(Request::new(BP_NOTIFICATION)).await
    }

    /// Returns a single notification.
    pub async fn get_notification(&self, notification_id: i32) -> Result<NotificationOutput> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_NOTIFICATION,
                &str_val(notification_id),
            ])))
            .await
    }

    /// Creates a notification.
    pub async fn add_notification(
        &self,
        notification: &NotificationInput,
    ) -> Result<NotificationOutput> {
        self.api
            .post_into(Request::new(BP_NOTIFICATION).with_json(notification)?)
            .await
    }

    /// Updates the notification.
    pub async fn update_notification(
        &self,
        notification: &NotificationInput,
    ) -> Result<NotificationOutput> {
        let req = Request::new(path_join(&[BP_NOTIFICATION, &str_val(notification.id)]))
            .with_json(notification)?;
        self.api.put_into(req).await
    }

    /// Removes a single notification.
    pub async fn delete_notification(&self, notification_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_NOTIFICATION,
                &str_val(notification_id),
            ])))
            .await
    }
}
