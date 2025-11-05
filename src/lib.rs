#![feature(lazy_get)]
mod global;
mod pool;
mod pstring;
pub use global::is_pool_init;
pub use pstring::PooledString;
