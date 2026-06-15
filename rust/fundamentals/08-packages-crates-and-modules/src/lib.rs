//! Fundamentals 08 — Packages, Crates & Modules.
//!
//! This crate's *organization* is the exercise: two submodules,
//! [`geometry`] and [`stats`], each holding their own `pub fn` stubs (full
//! rustdoc is on each function in its module file). [`stats::median`] is
//! additionally re-exported at the crate root via `pub use`, so it's also
//! reachable as `crate::median`.

pub mod geometry;
pub mod stats;

pub use stats::median;
