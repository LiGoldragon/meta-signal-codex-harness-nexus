use signal::Following;

use crate::{Configuration, ConfigurationChange, ConfigurationReadout, Set};

impl Following for Configuration {
    type Change = Set;

    fn after(mut state: Self, change: Self::Change) -> Self {
        match change {
            Set::TextBudget(value) => state.text_budget_option = Some(value),
            Set::FlowsRoot(value) => state.flows_root_option = Some(value),
            Set::IdentityProducer(value) => state.identity_producer_option = Some(value),
            Set::SessionsDirectories(value) => state.sessions_directories_option = Some(value),
            Set::ThreadDatabases(value) => state.thread_databases_option = Some(value),
            Set::SubscriberCapacity(value) => state.capacity_option = Some(value),
        }
        state
    }
}

/// The readout follows each value set, each change in which of the
/// recorded sessions directories are still awaited, and each change in
/// which cannot be watched.
impl Following for ConfigurationReadout {
    type Change = ConfigurationChange;

    fn after(mut state: Self, change: Self::Change) -> Self {
        match change {
            ConfigurationChange::Set(set) => {
                state.configuration = Configuration::after(state.configuration, set);
            }
            ConfigurationChange::AwaitedDirectories(awaited) => {
                state.awaited_directories = awaited;
            }
            ConfigurationChange::UnwatchableDirectories(unwatchable) => {
                state.unwatchable_directories = unwatchable;
            }
        }
        state
    }
}
