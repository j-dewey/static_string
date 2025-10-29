// This will be the primary interface for interacting with the pool.
// Whenever a new StaticStr is made, it will use the methods in `global.rs`
// to safely update the pool.
#[derive(Clone, Copy, Debug, Hash)]
pub struct StaticStr;
