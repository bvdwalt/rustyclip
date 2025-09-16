//! Library for rustyclip.
//
// This crate is primarily a binary, but this minimal library allows docs.rs to build documentation.

/// Returns the version of the rustyclip application.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
