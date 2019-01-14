//! A collection of structures and functions useful across the entire amethyst project.

#![warn(missing_docs, rust_2018_idioms, rust_2018_compatibility)]

#[cfg(all(target_os = "emscripten", not(no_threading)))]
compile_error!("the cfg flag \"no_threading\" is required when building for emscripten");

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde;

#[macro_use]
#[cfg(feature = "profiler")]
extern crate thread_profiler;

pub use approx;
pub use nalgebra;
pub use shred;
pub use shrev;
pub use specs;

pub use crate::{
    bundle::{Error, ErrorKind, Result, SystemBundle},
    event::EventReader,
    system_ext::{Pausable, SystemExt},
    timing::*,
    transform::*,
};

pub use self::{
    axis::{Axis2, Axis3},
    named::{Named, WithNamed},
};

use std::sync::Arc;

use rayon;

pub mod bundle;
pub mod frame_limiter;
pub mod timing;
pub mod transform;

mod axis;
mod event;
mod named;
mod system_ext;

/// A rayon thread pool wrapped in an `Arc`. This should be used as resource in `World`.
pub type ArcThreadPool = Arc<rayon::ThreadPool>;
