// buggy: https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+derive_partial_eq_without_eq
#![allow(clippy::derive_partial_eq_without_eq)]

mod battery_state;
mod error;
mod fail_status;
pub mod uart;
mod utils;

pub use battery_state::BatteryState;
pub use error::{Error, Result};
pub use fail_status::{FailStatus, FailStatusItem, FailState};

