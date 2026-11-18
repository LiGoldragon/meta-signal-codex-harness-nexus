use signal::Following;

use crate::{Configuration, Set};

impl Following for Configuration {
    type Change = Set;

    fn after(mut state: Self, change: Self::Change) -> Self {
        match change {
            Set::FlowsRoot(value) => state.flows_root_option = Some(value),
            Set::IdentityProducer(value) => state.identity_producer_option = Some(value),
            Set::SessionsDirectories(value) => state.sessions_directories_option = Some(value),
            Set::ThreadDatabases(value) => state.thread_databases_option = Some(value),
            Set::SubscriberCapacity(value) => state.capacity_option = Some(value),
        }
        state
    }
}
