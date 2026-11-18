//! The Codex Harness Nexus's meta vocabulary: its configuration, set one
//! value at a time, and the subscription that follows it.

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
