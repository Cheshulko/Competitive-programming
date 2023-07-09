use std::collections::HashSet;

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut x = HashSet::<i32>::new();

        for g in nums.into_iter() {
            x.insert(g);
        }

        for (ind, m) in move_from.iter().enumerate() {
            let to = move_to[ind];

            x.remove(m);
            x.insert(to);
        }

        let mut y = x.into_iter().collect::<Vec<_>>();
        y.sort();

        y
    }
}
