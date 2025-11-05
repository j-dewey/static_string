use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::{pool::StringPool, pstring::PooledString};

type GlobalPool = Mutex<StringPool>;
static GLOBAL_STRING_POOL: LazyLock<GlobalPool> = LazyLock::new(|| Mutex::new(StringPool::new()));

pub fn is_pool_init() -> bool {
    LazyLock::get(&GLOBAL_STRING_POOL).is_some()
}

// Copy a [&str] into the StringPool
pub(crate) fn make_static(s: &str) -> PooledString {
    let mut pool_guard: MutexGuard<'static, StringPool> =
        GLOBAL_STRING_POOL.lock().expect("String pool poisoned");
    let refr = pool_guard.store(s);
    refr
}
