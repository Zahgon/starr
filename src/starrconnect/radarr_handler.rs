use super::{
    Callback, ErrorCallback, EventType, HandlerFailure, MovieAdded, MovieDelete, MovieFileDelete,
    RadarrApplicationUpdate, RadarrDownload, RadarrEvent, RadarrGrab, RadarrHealth,
    RadarrManualInteraction, RadarrRename, WebhookError, WebhookResponse, check_body_size,
    parse_radarr, run_webhook_callback,
};
use std::fmt;

/// RadarrHandler dispatches Radarr webhook POSTs to per-event callbacks.
///
/// Assign the callbacks you care about, then feed requests to
/// [`RadarrHandler::handle`] from your HTTP server.
#[derive(Default)]
pub struct RadarrHandler {
    /// Called for Grab events.
    pub on_grab: Option<Callback<RadarrGrab>>,
    /// Called for Download events.
    pub on_download: Option<Callback<RadarrDownload>>,
    /// Called for Rename events.
    pub on_rename: Option<Callback<RadarrRename>>,
    /// Called for MovieAdded events.
    pub on_movie_added: Option<Callback<MovieAdded>>,
    /// Called for MovieDelete events.
    pub on_movie_delete: Option<Callback<MovieDelete>>,
    /// Called for MovieFileDelete events.
    pub on_movie_file_delete: Option<Callback<MovieFileDelete>>,
    /// Called for Health events.
    pub on_health: Option<Callback<RadarrHealth>>,
    /// Called for HealthRestored events.
    pub on_health_restored: Option<Callback<RadarrHealth>>,
    /// Called for ApplicationUpdate events.
    pub on_application_update: Option<Callback<RadarrApplicationUpdate>>,
    /// Called for ManualInteractionRequired events.
    pub on_manual_interaction_required: Option<Callback<RadarrManualInteraction>>,
    /// Called for Test events, which use the Grab shape.
    pub on_test: Option<Callback<RadarrGrab>>,
    /// Called with the cause whenever a webhook fails.
    pub on_error: Option<ErrorCallback>,
}

impl fmt::Debug for RadarrHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RadarrHandler").finish_non_exhaustive()
    }
}

impl RadarrHandler {
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
            parse_radarr(body).map_err(|err| HandlerFailure::new(400, "invalid json", err))?;

        self.dispatch_event(&event)
    }

    fn dispatch_event(&self, event: &RadarrEvent) -> Result<(), HandlerFailure> {
        let event_type = &event.base.event_type;

        if *event_type == EventType::TEST {
            run_webhook_callback(self.on_test.as_ref(), || event.get_grab())
        } else if *event_type == EventType::GRAB {
            run_webhook_callback(self.on_grab.as_ref(), || event.get_grab())
        } else if *event_type == EventType::DOWNLOAD {
            run_webhook_callback(self.on_download.as_ref(), || event.get_download())
        } else if *event_type == EventType::RENAME {
            run_webhook_callback(self.on_rename.as_ref(), || event.get_rename())
        } else if *event_type == EventType::MOVIE_ADDED {
            run_webhook_callback(self.on_movie_added.as_ref(), || event.get_movie_added())
        } else if *event_type == EventType::MOVIE_DELETE {
            run_webhook_callback(self.on_movie_delete.as_ref(), || event.get_movie_delete())
        } else if *event_type == EventType::MOVIE_FILE_DELETE {
            run_webhook_callback(self.on_movie_file_delete.as_ref(), || {
                event.get_movie_file_delete()
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
}
