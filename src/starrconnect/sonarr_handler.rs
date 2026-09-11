use super::{
    Callback, ErrorCallback, EventType, HandlerFailure, SeriesAdd, SeriesDelete,
    SonarrApplicationUpdate, SonarrDownload, SonarrEvent, SonarrGrab, SonarrHealth,
    SonarrImportComplete, SonarrManualInteraction, SonarrRename, WebhookError, WebhookResponse,
    check_body_size, parse_sonarr, run_webhook_callback, sonarr_is_import_complete_body,
};
use super::EpisodeFileDelete;
use std::fmt;

/// SonarrHandler dispatches Sonarr webhook POSTs to per-event callbacks.
///
/// Assign the callbacks you care about, then feed requests to
/// [`SonarrHandler::handle`] from your HTTP server.
#[derive(Default)]
pub struct SonarrHandler {
    /// Called for Grab events.
    pub on_grab: Option<Callback<SonarrGrab>>,
    /// Called for single-file Download events.
    pub on_download: Option<Callback<SonarrDownload>>,
    /// Called for batch import-complete Download events.
    pub on_import_complete: Option<Callback<SonarrImportComplete>>,
    /// Called for Rename events.
    pub on_rename: Option<Callback<SonarrRename>>,
    /// Called for SeriesAdd events.
    pub on_series_add: Option<Callback<SeriesAdd>>,
    /// Called for SeriesDelete events.
    pub on_series_delete: Option<Callback<SeriesDelete>>,
    /// Called for EpisodeFileDelete events.
    pub on_episode_file_delete: Option<Callback<EpisodeFileDelete>>,
    /// Called for Health events.
    pub on_health: Option<Callback<SonarrHealth>>,
    /// Called for HealthRestored events.
    pub on_health_restored: Option<Callback<SonarrHealth>>,
    /// Called for ApplicationUpdate events.
    pub on_application_update: Option<Callback<SonarrApplicationUpdate>>,
    /// Called for ManualInteractionRequired events.
    pub on_manual_interaction_required: Option<Callback<SonarrManualInteraction>>,
    /// Called for Test events, which use the Grab shape.
    pub on_test: Option<Callback<SonarrGrab>>,
    /// Called with the cause whenever a webhook fails.
    pub on_error: Option<ErrorCallback>,
}

impl fmt::Debug for SonarrHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SonarrHandler").finish_non_exhaustive()
    }
}

impl SonarrHandler {
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
            parse_sonarr(body).map_err(|err| HandlerFailure::new(400, "invalid json", err))?;

        self.dispatch_event(&event, body)
    }

    fn dispatch_event(&self, event: &SonarrEvent, body: &[u8]) -> Result<(), HandlerFailure> {
        let event_type = &event.base.event_type;

        if *event_type == EventType::TEST {
            run_webhook_callback(self.on_test.as_ref(), || event.get_grab())
        } else if *event_type == EventType::GRAB {
            run_webhook_callback(self.on_grab.as_ref(), || event.get_grab())
        } else if *event_type == EventType::DOWNLOAD {
            self.dispatch_download(event, body)
        } else if *event_type == EventType::RENAME {
            run_webhook_callback(self.on_rename.as_ref(), || event.get_rename())
        } else if *event_type == EventType::SERIES_ADD {
            run_webhook_callback(self.on_series_add.as_ref(), || event.get_series_add())
        } else if *event_type == EventType::SERIES_DELETE {
            run_webhook_callback(self.on_series_delete.as_ref(), || event.get_series_delete())
        } else if *event_type == EventType::EPISODE_FILE_DELETE {
            run_webhook_callback(self.on_episode_file_delete.as_ref(), || {
                event.get_episode_file_delete()
            })
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
        } else if *event_type == EventType::MANUAL_INTERACTION_REQUIRED {
            run_webhook_callback(self.on_manual_interaction_required.as_ref(), || {
                event.get_manual_interaction()
            })
        } else {
            Err(HandlerFailure::new(
                500,
                "handler error",
                WebhookError::UnknownEvent(event_type.clone()),
            ))
        }
    }

    fn dispatch_download(
        &self,
        event: &SonarrEvent,
        body: &[u8],
    ) -> Result<(), HandlerFailure> {
        if sonarr_is_import_complete_body(body) {
            return run_webhook_callback(self.on_import_complete.as_ref(), || {
                event.get_import_complete()
            });
        }

        run_webhook_callback(self.on_download.as_ref(), || event.get_download())
    }
}
