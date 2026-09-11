use super::Prowlarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use serde::{Deserialize, Serialize};

const BP_NOTIFICATION: &str = "v1/notification";

/// NotificationInput is the input for a new or updated notification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationInput {
    /// Notify on grab.
    #[serde(default)]
    pub on_grab: bool,
    /// Notify on health issue.
    #[serde(default)]
    pub on_health_issue: bool,
    /// Notify when a health issue is resolved.
    #[serde(default)]
    pub on_health_restored: bool,
    /// Notify on application update.
    #[serde(default)]
    pub on_application_update: bool,
    /// Whether the implementation supports grab notifications.
    #[serde(default)]
    pub supports_on_grab: bool,
    /// Whether manual grabs raise notifications.
    #[serde(default)]
    pub include_manual_grabs: bool,
    /// Whether the implementation supports health issue notifications.
    #[serde(default)]
    pub supports_on_health_issue: bool,
    /// Whether the implementation supports health restored notifications.
    #[serde(default)]
    pub supports_on_health_restored: bool,
    /// Whether health warnings raise notifications.
    #[serde(default)]
    pub include_health_warnings: bool,
    /// Whether the implementation supports application update notifications.
    #[serde(default)]
    pub supports_on_application_update: bool,
    /// Notification ID. Used on update only.
    #[serde(default, skip_serializing_if = "is_default")]
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
    /// Link to more information about the implementation.
    #[serde(default)]
    pub info_link: String,
    /// Tags applied to this notification.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldInput>,
}

/// NotificationMessage is the message block embedded in [`NotificationOutput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationMessage {
    /// The message text.
    #[serde(default)]
    pub message: String,
    /// The message type.
    #[serde(default, rename = "type")]
    pub message_type: String,
}

/// NotificationOutput is the output from the notification methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationOutput {
    /// Notify on grab.
    #[serde(default)]
    pub on_grab: bool,
    /// Notify on health issue.
    #[serde(default)]
    pub on_health_issue: bool,
    /// Notify when a health issue is resolved.
    #[serde(default)]
    pub on_health_restored: bool,
    /// Notify on application update.
    #[serde(default)]
    pub on_application_update: bool,
    /// Whether the implementation supports grab notifications.
    #[serde(default)]
    pub supports_on_grab: bool,
    /// Whether manual grabs raise notifications.
    #[serde(default)]
    pub include_manual_grabs: bool,
    /// Whether the implementation supports health issue notifications.
    #[serde(default)]
    pub supports_on_health_issue: bool,
    /// Whether the implementation supports health restored notifications.
    #[serde(default)]
    pub supports_on_health_restored: bool,
    /// Whether health warnings raise notifications.
    #[serde(default)]
    pub include_health_warnings: bool,
    /// Whether the implementation supports application update notifications.
    #[serde(default)]
    pub supports_on_application_update: bool,
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
    /// Link to more information about the implementation.
    #[serde(default)]
    pub info_link: String,
    /// Tags applied to this notification.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
    /// Status message. This is a weird place for a message.
    #[serde(default)]
    pub message: NotificationMessage,
}

impl Prowlarr {
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
