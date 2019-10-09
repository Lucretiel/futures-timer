//! A general purpose crate for working with timeouts and delays with futures.
//!
//! This crate is intended to provide general purpose timeouts and interval
//! streams for working with `futures`.
//!
//! Basic usage of this crate is relatively simple:
//!
//! ```no_run
//! # #[runtime::main]
//! # async fn main() {
//! use std::time::Duration;
//! use futures_timer::Delay;
//! use futures::prelude::*;
//!
//! let now = Delay::new(Duration::from_secs(3)).await;
//! println!("waited for 3 secs");
//! # }
//! ```
//!
//! And you're off to the races!

#![deny(missing_docs)]
#![warn(missing_debug_implementations)]

mod arc_list;
mod delay;
mod global;
mod heap;
mod heap_timer;
mod timer;

use arc_list::{ArcList, Node};
use heap::{Heap, Slot};
use heap_timer::HeapTimer;
use timer::{ScheduledTimer, Timer, TimerHandle};

pub use self::delay::Delay;
