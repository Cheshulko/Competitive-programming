// https://leetcode.com/problems/last-stone-weight

struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        while stones.len() > 1 {
            stones.sort();
            let x = stones.pop().unwrap();
            let y = stones.pop().unwrap();

            stones.push((x - y).abs())
        }

        stones[0]
    }
}
