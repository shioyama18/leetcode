use std::collections::BTreeMap;

struct MyHashSet {
    buckets: Vec<BTreeMap<i32, Option<i32>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    const NUM_BUCKETS: usize = 769;

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            buckets: vec![BTreeMap::new(); Self::NUM_BUCKETS],
        }
    }

    fn add(&mut self, key: i32) {
        let index = Self::get_index(key);
        if !self.buckets[index].contains_key(&key) {
            self.buckets[index].insert(key, None);
        }
    }

    fn remove(&mut self, key: i32) {
        let index = Self::get_index(key);
        self.buckets[index].remove(&key);
    }

    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        let index = Self::get_index(key);
        self.buckets[index].contains_key(&key)
    }

    fn get_index(key: i32) -> usize {
        key as usize % Self::NUM_BUCKETS
    }
}
