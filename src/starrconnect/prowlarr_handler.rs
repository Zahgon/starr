use super::{
    Callback, ErrorCallback, EventType, HandlerFailure, ProwlarrApplicationUpdate,
    ProwlarrDownload, ProwlarrEvent, ProwlarrGrab, ProwlarrHealth, ProwlarrRename, ProwlarrTest,
    WebhookError, WebhookResponse, check_body_size, parse_prowlarr, run_webhook_callback,
};
use std::fmt;

/// ProwlarrHandler dispatches Prowlarr webhook POSTs to per-event callbacks.
///
/// Assign the callbacks you care about, then feed requests to
/// [`ProwlarrHandler::handle`] from your HTTP server.
#[derive(Default)]
pub struct ProwlarrHandler {
    /// Called for Grab events.
    pub on_grab: Option<Callback<ProwlarrGrab>>,
    /// Called for Test events.
    pub on_test: Option<Callback<ProwlarrTest>>,
    /// Called for Download events.
    pub on_download: Option<Callback<ProwlarrDownload>>,
    /// Called for Rename events.
    pub on_rename: Option<Callback<ProwlarrRename>>,
    /// Called for Health events.
    pub on_health: Option<Callback<ProwlarrHealth>>,
    /// Called for HealthRestored events.
    pub on_health_restored: Option<Callback<ProwlarrHealth>>,
    /// Called for ApplicationUpdate events.
    pub on_application_update: Option<Callback<ProwlarrApplicationUpdate>>,
    /// Called with the cause whenever a webhook fails.
    pub on_error: Option<ErrorCallback>,
}

impl fmt::Debug for ProwlarrHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProwlarrHandler").finish_non_exhaustive()
    }
}

impl ProwlarrHandler {
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
            parse_prowlarr(body).map_err(|err| HandlerFailure::new(400, "invalid json", err))?;

        self.dispatch_event(&event)
    }

    fn dispatch_event(&self, event: &ProwlarrEvent) -> Result<(), HandlerFailure> {
        let event_type = &event.base.event_type;

        if *event_type == EventType::TEST {
            run_webhook_callback(self.on_test.as_ref(), || event.get_test())
        } else if *event_type == EventType::GRAB {
            run_webhook_callback(self.on_grab.as_ref(), || event.get_grab())
        } else if *event_type == EventType::DOWNLOAD {
            run_webhook_callback(self.on_download.as_ref(), || event.get_download())
        } else if *event_type == EventType::RENAME {
            run_webhook_callback(self.on_rename.as_ref(), || event.get_rename())
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
