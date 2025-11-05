use std::fmt::Display;

use crate::global;

// This will be the primary interface for interacting with the pool.
// Whenever a new StaticStr is made, it will use the methods in `global.rs`
// to safely update the pool.
//
#[derive(Clone, Copy, Hash)]
pub struct PooledString {
    pub(crate) raw: &'static str,
}

impl PooledString {
    // Create a StaticString from a [&'static str] without moving
    // or copying the string.
    //
    // NOTE: Since s is stored in static memory, it's readonly
    pub const fn from_static_str(s: &'static str) -> Self {
        Self { raw: s }
    }

    // Create a StaticString from any [&str]. This will clone the
    // slice into the static hashmap.
    pub fn from_str(s: &str) -> Self {
        global::make_static(s)
    }
}

impl Display for PooledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.raw.fmt(f)
    }
}
