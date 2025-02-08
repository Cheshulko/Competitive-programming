// https://leetcode.com/problems/design-a-number-container-system

use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    indx: HashMap<i32, i32>,
    nums: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            indx: HashMap::new(),
            nums: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        self.indx
            .remove(&index)
            .map(|prev| self.nums.get_mut(&prev).unwrap().remove(&index));

        self.indx.insert(index, number);
        self.nums.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        self.nums
            .get(&number)
            .and_then(|indx| indx.first().copied())
            .unwrap_or(-1)
    }
}
