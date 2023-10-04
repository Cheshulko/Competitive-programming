use std::collections::LinkedList;

struct MyHashMap {
    buckets: Vec<LinkedList<(i32, i32)>>,
}

impl MyHashMap {
    const MAX: usize = 1_000_000;
    const PRIME: usize = 11483;

    fn new() -> Self {
        MyHashMap {
            buckets: vec![LinkedList::new(); MyHashMap::PRIME],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let hash = key as usize % MyHashMap::PRIME;
        if let Some(has) = self.buckets[hash].iter_mut().find(|x| x.0 == key) {
            *has = (key, value);
        } else {
            self.buckets[hash].push_back((key, value));
        }
    }

    fn get(&self, key: i32) -> i32 {
        let hash = key as usize % MyHashMap::PRIME;
        if let Some(has) = self.buckets[hash].iter().find(|x| x.0 == key) {
            return has.1;
        } else {
            return -1;
        }
    }

    fn remove(&mut self, key: i32) {
        let hash = key as usize % MyHashMap::PRIME;
        if let Some(has) = self.buckets[hash].iter_mut().find
            /*F*CK linked list doest have remove func in stable?? WTF*/(|x| x.0 == key)
        {
            *has = (key, -1);
        }
    }
}
