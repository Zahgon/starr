use super::{
    AlbumDelete, ArtistAdd, ArtistDelete, Callback, ErrorCallback, EventType, HandlerFailure,
    LidarrApplicationUpdate, LidarrDownload, LidarrDownloadFailure, LidarrEvent, LidarrGrab,
    LidarrHealth, LidarrImportFailure, LidarrRename, LidarrRetag, WebhookError, WebhookResponse,
    check_body_size, parse_lidarr, run_webhook_callback,
};
use std::fmt;

/// LidarrHandler dispatches Lidarr webhook POSTs to per-event callbacks.
///
/// Assign the callbacks you care about, then feed requests to
/// [`LidarrHandler::handle`] from your HTTP server.
#[derive(Default)]
pub struct LidarrHandler {
    /// Called for Grab events.
    pub on_grab: Option<Callback<LidarrGrab>>,
    /// Called for Download events.
    pub on_download: Option<Callback<LidarrDownload>>,
    /// Called for DownloadFailure events.
    pub on_download_failure: Option<Callback<LidarrDownloadFailure>>,
    /// Called for ImportFailure events.
    pub on_import_failure: Option<Callback<LidarrImportFailure>>,
    /// Called for Rename events.
    pub on_rename: Option<Callback<LidarrRename>>,
    /// Called for Retag events.
    pub on_retag: Option<Callback<LidarrRetag>>,
    /// Called for ArtistAdd events.
    pub on_artist_add: Option<Callback<ArtistAdd>>,
    /// Called for ArtistDelete events.
    pub on_artist_delete: Option<Callback<ArtistDelete>>,
    /// Called for AlbumDelete events.
    pub on_album_delete: Option<Callback<AlbumDelete>>,
    /// Called for Health events.
    pub on_health: Option<Callback<LidarrHealth>>,
    /// Called for HealthRestored events.
    pub on_health_restored: Option<Callback<LidarrHealth>>,
    /// Called for ApplicationUpdate events.
    pub on_application_update: Option<Callback<LidarrApplicationUpdate>>,
    /// Called for Test events, which use the Grab shape.
    pub on_test: Option<Callback<LidarrGrab>>,
    /// Called with the cause whenever a webhook fails.
    pub on_error: Option<ErrorCallback>,
}

impl fmt::Debug for LidarrHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LidarrHandler").finish_non_exhaustive()
    }
}

impl LidarrHandler {
    /// Handles one webhook request and returns the response to write back.
    pub fn handle(&self, method: &str, body: &[u8]) -> WebhookResponse {
        if !method.eq_ignore_ascii_case("POST") {
            return WebhookResponse::error(405, "method not allowed");
        }

        match self.handle_webhook(body) {
            Ok(()) => WebhookResponse::ok(),
            Err(failure) => {
                if let Some(on_error) = &self.on_error {
                    on_error(failure.cause.as_ref());
                }

                failure.response
            }
        }
    }

    fn handle_webhook(&self, body: &[u8]) -> Result<(), HandlerFailure> {
        check_body_size(body).map_err(|err| HandlerFailure::new(400, "bad request", err))?;

        let event =
            parse_lidarr(body).map_err(|err| HandlerFailure::new(400, "invalid json", err))?;

        self.dispatch_event(&event)
    }

    fn dispatch_event(&self, event: &LidarrEvent) -> Result<(), HandlerFailure> {
        let event_type = &event.base.event_type;

        if *event_type == EventType::TEST {
            run_webhook_callback(self.on_test.as_ref(), || event.get_grab())
        } else if *event_type == EventType::GRAB {
            run_webhook_callback(self.on_grab.as_ref(), || event.get_grab())
        } else if *event_type == EventType::DOWNLOAD {
            run_webhook_callback(self.on_download.as_ref(), || event.get_download())
        } else if *event_type == EventType::DOWNLOAD_FAILURE {
            run_webhook_callback(self.on_download_failure.as_ref(), || {
                event.get_download_failure()
            })
        } else if *event_type == EventType::IMPORT_FAILURE {
            run_webhook_callback(self.on_import_failure.as_ref(), || {
                event.get_import_failure()
            })
        } else if *event_type == EventType::RENAME {
            run_webhook_callback(self.on_rename.as_ref(), || event.get_rename())
        } else if *event_type == EventType::RETAG {
            run_webhook_callback(self.on_retag.as_ref(), || event.get_retag())
        } else if *event_type == EventType::ARTIST_ADD {
            run_webhook_callback(self.on_artist_add.as_ref(), || event.get_artist_add())
        } else if *event_type == EventType::ARTIST_DELETE {
            run_webhook_callback(self.on_artist_delete.as_ref(), || event.get_artist_delete())
        } else if *event_type == EventType::ALBUM_DELETE {
            run_webhook_callback(self.on_album_delete.as_ref(), || event.get_album_delete())
        } else if *event_type == EventType::HEALTH {
            run_webhook_callback(self.on_health.as_ref(), || event.get_health())
        } else if *event_type == EventType::HEALTH_RESTORED {
            run_webhook_callback(self.on_health_restored.as_ref(), || {
                event.get_health_restored()
            })
        } else if *event_type == EventType::APPLICATION_UPDATE {
            run_webhook_callback(self.on_application_update.as_ref(), || {
                event.get_application_update()
            })
        } else {
            Err(HandlerFailure::new(
                500,
                "handler error",
                WebhookError::UnknownEvent(event_type.clone()),
            ))
        }
    }
}
