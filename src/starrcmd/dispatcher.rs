use super::{CmdError, CmdEvent, CmdResult, Event};
use crate::helpers::App;
use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

/// A callback registered with a [`Dispatcher`].
pub type Hook = Box<dyn Fn(&CmdEvent) -> CmdResult<()> + Send + Sync>;

/// Dispatcher registers callbacks for Custom Script invocations.
///
/// Call [`Dispatcher::run`] to parse the environment with [`CmdEvent::new`] and
/// invoke matching handlers, or call [`Dispatcher::dispatch`] with a
/// [`CmdEvent`] from tests or custom wiring.
#[derive(Default)]
pub struct Dispatcher {
    /// Maps (app, event) to one or more callbacks; all matching callbacks run in order.
    hooks: Mutex<HashMap<(App, Event), Vec<Hook>>>,
    /// Invoked when no handlers are registered for the event's app and type.
    pub on_unknown: Option<Hook>,
}

impl fmt::Debug for Dispatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dispatcher").finish_non_exhaustive()
    }
}

impl Dispatcher {
    /// Returns an empty Dispatcher ready for [`Dispatcher::register`] or the
    /// typed `on_{app}_{event}` helpers.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a callback for the given app and event.
    ///
    /// Multiple registrations for the same pair are all run, in registration
    /// order, when [`Dispatcher::dispatch`] matches.
    pub fn register<F>(&self, app: App, event_type: Event, callback: F)
    where
        F: Fn(&CmdEvent) -> CmdResult<()> + Send + Sync + 'static,
    {
        let mut hooks = self.hooks.lock().expect("dispatcher hooks lock poisoned");
        hooks
            .entry((app, event_type))
            .or_default()
            .push(Box::new(callback));
    }

    /// Calls [`CmdEvent::new`], then [`Dispatcher::dispatch`] with the result.
    ///
    /// It returns [`CmdError::NoEventFound`] before callbacks run.
    pub fn run(&self) -> CmdResult<()> {
        let cmd = CmdEvent::new()?;
        self.dispatch(&cmd)
    }

    /// Runs all handlers registered for the event's app and type.
    ///
    /// If none match and `on_unknown` is set, its result is returned. If none
    /// match and `on_unknown` is unset, this returns `Ok(())`. The first
    /// callback error stops execution and is returned.
    pub fn dispatch(&self, cmd: &CmdEvent) -> CmdResult<()> {
        let hooks = self.hooks.lock().expect("dispatcher hooks lock poisoned");
        let key = (cmd.app.clone(), cmd.event_type.clone());

        let Some(callbacks) = hooks.get(&key) else {
            drop(hooks);

            return match &self.on_unknown {
                Some(on_unknown) => on_unknown(cmd),
                None => Ok(()),
            };
        };

        for callback in callbacks {
            callback(cmd)?;
        }

        Ok(())
    }
}

/// Parses the payload with `getter` and passes it to `handler`.
pub(crate) fn execute_get<T, G, H>(cmd: &CmdEvent, getter: G, handler: H) -> CmdResult<()>
where
    G: FnOnce(&CmdEvent) -> CmdResult<T>,
    H: Fn(&T) -> CmdResult<()>,
{
    let value = getter(cmd).map_err(|err| CmdError::ParsePayload(Box::new(err)))?;

    handler(&value)
}
