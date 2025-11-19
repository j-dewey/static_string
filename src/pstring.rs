use std::fmt::Display;

use crate::global;

// This will be the primary interface for interacting with the pool.
// Whenever a new PooledString is made, it will use the methods in `global.rs`
// to safely update the pool.
#[derive(Hash, std::fmt::Debug)]
pub struct PooledString {
    pub(crate) raw: &'static str,
    // Was this string made from a real static ptr
    // or is it stored in the pool?
    pub(crate) true_static: bool,
}

impl PooledString {
    // Create a StaticString from a [&'static str] without moving
    // or copying the string.
    //
    // NOTE: Since s is stored in static memory, it's readonly
    pub const fn from_static_str(s: &'static str) -> Self {
        Self {
            raw: s,
            true_static: true,
        }
    }

    // Create a StaticString from any [&str]. This will clone the
    // slice into the static hashmap.
    pub fn from_str(s: &str) -> Self {
        global::make_static(s)
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        self.raw
    }
}

impl Clone for PooledString {
    fn clone(&self) -> Self {
        global::clone_pstring(self)
    }
}

impl PartialEq<str> for PooledString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.raw == other
    }

    #[inline]
    fn ne(&self, other: &str) -> bool {
        self.raw != other
    }
}

impl Display for PooledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.raw.fmt(f)
    }
}
