//! The Codex Harness Nexus's meta vocabulary: its configuration, set one
//! value at a time, and the subscription that follows it.

mod generated {
    pub mod signal;
}
mod following;

pub use generated::signal::*;
