#![deny(unsafe_op_in_unsafe_fn)]

//! A minimal async runtime implementation for educational purposes.
//!
//! This crate provides a single-threaded, cooperative async runtime that
//! implements the core functionality of task spawning, execution, and timing.

mod executor;
mod task;
mod time;

pub use executor::Executor;
pub use task::JoinHandle;
pub use time::delay;

/// Error type for cancelled tasks
#[derive(Debug, Clone)]
pub struct Cancelled;

impl std::fmt::Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task was cancelled")
    }
}

impl std::error::Error for Cancelled {}