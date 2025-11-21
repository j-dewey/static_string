use std::{
    collections::HashMap,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::pstring::PooledString;

#[derive(Debug)]
pub(crate) struct StrEntry {
    raw: &'static str,
    count: AtomicUsize,
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
    heap_strings: HashMap<&'static str, StrEntry>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            heap_strings: HashMap::new(),
        }
    }

    // Move a string into the pool and get a reference to the stored string
    // if the string is already stored, the reference is still returned
    pub fn store(&mut self, string: &str) -> PooledString {
        let string = String::from(string);

        let pooled = match self.heap_strings.get(string.as_str()) {
            Some(s) => s,
            None => {
                let leaked: &mut str = Box::leak(Box::from(string));
                let entry = StrEntry {
                    raw: leaked,
                    count: AtomicUsize::new(1),
                };

                self.heap_strings.insert(leaked, entry);
                self.heap_strings
                    .get(leaked)
                    .expect("Unexecpectedly removed string from pool during string storage")
            }
        };

        PooledString {
            raw: pooled.raw,
            true_static: false,
        }
    }

    // Create a clone of a pooled string that was pooled already
    //  i.e. s.true_static == false
    pub fn clone_pooled(&mut self, s: &PooledString) -> Option<PooledString> {
        let ent = self.heap_strings.get_mut(s.as_str())?;
        ent.count.fetch_add(1, Ordering::Relaxed);
        Some(PooledString {
            raw: ent.raw,
            true_static: false,
        })
    }
}
