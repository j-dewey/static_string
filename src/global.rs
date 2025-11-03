use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::{pool::StringPool, static_str::StaticString};

type GlobalPool = Mutex<StringPool>;
static GLOBAL_STRING_POOL: LazyLock<GlobalPool> = LazyLock::new(|| Mutex::new(StringPool::new()));

// Copy a [&str] into the StringPool
pub(crate) fn make_static(s: &str) -> StaticString {
    let mut pool_guard: MutexGuard<'static, StringPool> =
        GLOBAL_STRING_POOL.lock().expect("String pool poisoned");
    let refr = pool_guard.store(s);
    refr
}
