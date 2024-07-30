//! tiny is a simple web framework for rust.
//!
//! tiny's core goal is to have as few dependencies as possible. _(which is currently 32)_
//!
//! while many rust web frameworks such as axum and rocket have alot of
//! features, quite easy to use and performant, tiny aims to achieve extremy
//! simplicity while providing as much performance as possible. which should
//! be somewhere around 120 000 requests/s _(although only for small sites)_.

#[cfg(feature = "cache")]
pub mod cache;
pub mod prelude;

#[cfg(feature = "attributes")]
pub use tiny_attributes as attr;
