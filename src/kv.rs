use std::cmp::Ordering;

use json::Json;

#[derive(Debug,Clone)]
pub struct KeyValue(String,Json);

impl KeyValue {
    #[inline]
    pub fn new(key: String, value: Json) -> KeyValue {
        KeyValue(key, value)
    }

    #[inline]
    pub fn key(self) -> String {
        self.0
    }

    #[inline]
    pub fn key_ref(&self) -> &String {
        &self.0
    }

    #[inline]
    pub fn value(self) -> Json {
        self.1
    }

    #[inline]
    pub fn value_ref(&self) -> &Json {
        &self.1
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut Json {
        &mut self.1
    }

    #[inline]
    pub fn set_value(&mut self, value: Json) {
        self.1 = value;
    }
}

// Eq, PartialEq and PartialOrd

impl Eq for KeyValue {}

impl PartialEq for KeyValue {
    fn eq(&self, other: &KeyValue) -> bool {
        self.0 == other.0 // compare only the key.
    }
}

impl PartialOrd for KeyValue {
    fn partial_cmp(&self, other: &KeyValue) -> Option<Ordering> {
        self.0.partial_cmp(other.key_ref()) // compare only the key.
    }
}


pub fn search_by_key(obj: &Vec<KeyValue>, key: &str) -> Result<usize,usize> {
    use std::cmp::Ordering::{Greater, Equal, Less};

    let mut size = obj.len();
    if size == 0 { return Err(0) }

    let mut base = 0_usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // mid is always in [0, size), that means mid is >= 0 and < size.
        // mid >= 0: by definition
        // mid < size: mid = size / 2 + size / 4 + size / 8 ...
        let item: &str = obj[mid].key_ref();
        let cmp = item.cmp(key);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    // base is always in [0, size) because base <= mid.
    let item: &str = obj[base].key_ref();
    let cmp = item.cmp(key);
    if cmp == Equal { Ok(base) } else { Err(base + (cmp == Less) as usize) }
}

pub fn upsert_object_key(obj: &mut Vec<KeyValue>, kv: KeyValue) {
    match search_by_key(obj, kv.key_ref()) {
        Ok(off) => obj[off] = kv,
        Err(off) => obj.insert(off, kv),
    }
}