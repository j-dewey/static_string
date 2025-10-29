use std::sync::OnceLock;

use crate::pool::StringPool;

static GLOBAL_STRING_POOL: OnceLock<StringPool> = OnceLock::new();

// Load a [&'static str] to the StringPool.
// Return None if the pool is uninit, otherwise return the new StaticStr back
pub(crate) const fn new_static(s: &'static str) -> Option<&'static str> {
    todo!()
}

// Copy a [&str] into the StringPool
// Return None if the pool is uninit, otherwise return the
pub(crate) fn make_static(s: &str) -> Option<&'static str> {
    todo!()
}
