use std::collections::HashSet;

pub(crate) struct StringPool {
    strings: HashSet<Box<str>>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            strings: HashSet::new(),
        }
    }
}
