use std::{hash::Hash, sync::atomic::AtomicUsize};

use scc::HashMap;

use crate::pstring::PooledString;

#[derive(Debug)]
pub(crate) struct StrEntry {
    _count: AtomicUsize,
    raw: &'static str,
}

impl PartialEq for StrEntry {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }

    fn ne(&self, other: &Self) -> bool {
        self.raw != other.raw
    }
}
impl Eq for StrEntry {}

impl Hash for StrEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }

    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for s in data.iter() {
            s.raw.hash(state);
        }
    }
}

pub(crate) struct StringPool {
    heap_strings: HashMap<usize, StrEntry>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            heap_strings: HashMap::new(),
        }
    }

    pub fn store(&mut self, s: &str) -> PooledString {
        let boxed: Box<str> = Box::from(s);
        let hash = boxed.as_ptr() as usize;

        let entry = StrEntry {
            _count: AtomicUsize::new(1),
            raw: Box::leak(boxed),
        };

        let entry = match self.heap_strings.insert_sync(hash, entry) {
            Ok(()) => self.heap_strings.get_sync(&hash),
            Err(_) => self.heap_strings.get_sync(&hash),
        }
        .expect("Failed to get static string after insertion into hash map");
        let string = entry.get();

        PooledString { raw: string.raw }
    }
}
