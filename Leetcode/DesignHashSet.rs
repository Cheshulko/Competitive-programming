// https://leetcode.com/problems/design-hashset

use std::collections::BTreeSet;

struct MyHashSet {
    // Should be LinkedList / Binary balanced tree
    buckets: Vec<BTreeSet<i32>>,
}

impl MyHashSet {
    const MOD: usize = 3947;
    fn new() -> Self {
        MyHashSet {
            buckets: vec![BTreeSet::<i32>::new(); MyHashSet::MOD],
        }
    }

    fn add(&mut self, key: i32) {
        let bucket = (key as usize) % MyHashSet::MOD;
        self.buckets[bucket].insert(key);
    }

    fn remove(&mut self, key: i32) {
        let bucket = (key as usize) % MyHashSet::MOD;
        self.buckets[bucket].remove(&key);
    }

    fn contains(&self, key: i32) -> bool {
        let bucket = (key as usize) % MyHashSet::MOD;
        self.buckets[bucket].contains(&key)
    }
}
