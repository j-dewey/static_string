use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::{pool::StringPool, pstring::PooledString};

type GlobalPool = Mutex<StringPool>;
static GLOBAL_STRING_POOL: LazyLock<GlobalPool> = LazyLock::new(|| Mutex::new(StringPool::new()));

// Check if the global string pool is init
//
// NOTE: This was made for testing and will likely not be needed
pub fn is_pool_init() -> bool {
    LazyLock::get(&GLOBAL_STRING_POOL).is_some()
}

// Copy a [&str] into the StringPool
pub(crate) fn make_static(s: &str) -> PooledString {
    let mut pool_guard: MutexGuard<'static, StringPool> =
        GLOBAL_STRING_POOL.lock().expect("String pool poisoned");
    pool_guard.store(s)
}

pub(crate) fn clone_pstring(s: &PooledString) -> PooledString {
    if s.true_static {
        return PooledString {
            raw: s.raw,
            true_static: s.true_static,
        };
    }

    let mut pool_guard: MutexGuard<'static, StringPool> =
        GLOBAL_STRING_POOL.lock().expect("String pool poisoned");

    pool_guard.clone_pooled(s)
}
