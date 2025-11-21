use std::{
    collections::HashMap,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::pstring::PooledString;

// ----------------------------
// | entry 1 | entry 2 | .... |
// ----------------------------
//  <---String 1--->   <-------String 2------->   ....
// -----------------------------------------------------
// | char 1 | char 2 | char 3 | char 4 | char 5 | .... |
// -----------------------------------------------------

// SAFETY:
//      This function moves ownership of s, so this should not be called on
//      any str that is really owned by another object. Also, the str should not
//      really be static otherwise this will cause a segfault / UB
unsafe fn drop_leaked_string(s: *mut u8, len: usize, cap: usize) {
    let _ = unsafe { String::from_raw_parts(s as *mut u8, len, cap) };
}

#[derive(Debug)]
pub(crate) struct StrEntry {
    raw: &'static str,
    cap: usize,
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
        let str_cap = string.capacity(); // may have access to more memory than the length
        let leaked: &mut str = Box::leak(Box::from(string));

        let string = match self.heap_strings.get(leaked) {
            Some(s) => {
                // (leaked) isn't being tracked anymore, so need to reclaim the memory to avoid
                // a memory leak
                // SAFETY:
                //      The region owned by leaked was first owned by string, however string never
                //      had its destructor called. Since that region was owned by a String, it must be
                //      allocated on the heap.
                unsafe { drop_leaked_string(leaked.as_mut_ptr(), leaked.len(), str_cap) };
                s
            }
            None => {
                let entry = StrEntry {
                    raw: leaked,
                    cap: str_cap,
                    count: AtomicUsize::new(1),
                };
                self.heap_strings.insert(leaked, entry);
                self.heap_strings
                    .get(leaked)
                    .expect("Unexecpectedly removed string from pool during string storage")
            }
        };

        PooledString {
            raw: string.raw,
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
