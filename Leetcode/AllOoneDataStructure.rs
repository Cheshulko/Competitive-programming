use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct AllOne {
    max: BinaryHeap<(usize, usize)>,
    min: BinaryHeap<(Reverse<usize>, usize)>,
    hm: HashMap<String, usize>,
    cnt: Vec<usize>,
    strs: Vec<String>,
}

impl AllOne {
    fn new() -> Self {
        Self {
            max: BinaryHeap::new(),
            min: BinaryHeap::new(),
            hm: HashMap::new(),
            cnt: Vec::new(),
            strs: Vec::new(),
        }
    }

    fn inc(&mut self, key: String) {
        let ind = if let Some(&ind) = self.hm.get(&key) {
            ind
        } else {
            let ind = self.hm.len();
            self.hm.insert(key.clone(), ind);

            ind
        };
        if self.cnt.len() <= ind {
            self.cnt.push(1);
            self.strs.push(key);
        } else {
            self.cnt[ind] += 1
        }

        self.max.push((self.cnt[ind], ind));
        self.min.push((Reverse(self.cnt[ind]), ind));
    }

    fn dec(&mut self, key: String) {
        if let Some(&ind) = self.hm.get(&key) {
            self.cnt[ind] -= 1;
        }
    }

    fn get_max_key(&mut self) -> String {
        while let Some(&(cnt, ind)) = self.max.peek() {
            if self.cnt[ind] != cnt {
                self.max.pop();
            } else {
                if let Some(str) = self.strs.get(ind) {
                    return str.clone();
                } else {
                    unreachable!()
                }
            }
        }
        String::new()
    }

    fn get_min_key(&mut self) -> String {
        while let Some(&(Reverse(cnt), ind)) = self.min.peek() {
            if self.cnt[ind] != cnt {
                self.min.pop();
            } else {
                if let Some(str) = self.strs.get(ind) {
                    return str.clone();
                } else {
                    unreachable!()
                }
            }
        }
        String::new()
    }
}
