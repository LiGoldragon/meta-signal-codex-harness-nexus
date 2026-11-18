//! The Codex Harness Nexus's meta vocabulary: its configuration, set one
//! value at a time, and the subscription that follows its readout. The text
//! budget is the translator's own setting, which the translator answers
//! itself; every other setting is the Nexus's. The readout is the recorded
//! configuration and the sessions directories recorded but not yet present,
//! which the Nexus awaits; a directory present that cannot be watched is
//! refused, never recorded.

mod generated {
    pub mod signal;
}
mod following;

pub use generated::signal::*;

/// The contract's authored Ethos, whose digest both ends of a connection
/// greet with.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

impl signal::Contracted for Input {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
