use super::{CmdEvent, CmdResult, Dispatcher, Event, execute_get};
use crate::env_struct;
use crate::helpers::App;

env_struct! {
    /// ProwlarrApplicationUpdate is the ApplicationUpdate event.
    pub struct ProwlarrApplicationUpdate {
        /// 4.0.3.5875
        previous_version: String = "prowlarr_update_previousversion";
        /// 4.0.4.5909
        new_version: String = "prowlarr_update_newversion";
        /// Prowlarr updated from 4.0.3.5875 to 4.0.4.5909
        message: String = "prowlarr_update_message";
    }
}

env_struct! {
    /// ProwlarrHealthIssue is the HealthIssue event.
    pub struct ProwlarrHealthIssue {
        /// some message about some problem
        message: String = "prowlarr_health_issue_message";
        /// NeverSeenOne
        issue_type: String = "prowlarr_health_issue_type";
        /// something
        wiki: String = "prowlarr_health_issue_wiki";
        /// Warning
        level: String = "prowlarr_health_issue_level";
    }
}

env_struct! {
    /// ProwlarrTest has no members.
    pub struct ProwlarrTest {
    }
}

impl CmdEvent {
    /// returns the ApplicationUpdate event data.
    pub fn get_prowlarr_application_update(&self) -> CmdResult<ProwlarrApplicationUpdate> {
        self.check(&Event::APPLICATION_UPDATE)?;
        ProwlarrApplicationUpdate::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_prowlarr_health_issue(&self) -> CmdResult<ProwlarrHealthIssue> {
        self.check(&Event::HEALTH_ISSUE)?;
        ProwlarrHealthIssue::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_prowlarr_test(&self) -> CmdResult<ProwlarrTest> {
        self.check(&Event::TEST)?;
        ProwlarrTest::from_env()
    }
}

impl Dispatcher {
    /// registers a Prowlarr ApplicationUpdate callback.
    pub fn on_prowlarr_application_update<F>(&self, handler: F)
    where
        F: Fn(&ProwlarrApplicationUpdate) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::PROWLARR, Event::APPLICATION_UPDATE, move |cmd| {
            execute_get(cmd, CmdEvent::get_prowlarr_application_update, &handler)
        });
    }

    /// registers a Prowlarr HealthIssue callback.
    pub fn on_prowlarr_health_issue<F>(&self, handler: F)
    where
        F: Fn(&ProwlarrHealthIssue) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::PROWLARR, Event::HEALTH_ISSUE, move |cmd| {
            execute_get(cmd, CmdEvent::get_prowlarr_health_issue, &handler)
        });
    }

    /// registers a Prowlarr Test callback.
    pub fn on_prowlarr_test<F>(&self, handler: F)
    where
        F: Fn(&ProwlarrTest) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::PROWLARR, Event::TEST, move |cmd| {
            execute_get(cmd, CmdEvent::get_prowlarr_test, &handler)
        });
    }
}
