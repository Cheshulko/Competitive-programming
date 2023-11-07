// https://leetcode.com/problems/eliminate-maximum-number-of-monsters
// https://leetcode.com/problems/eliminate-maximum-number-of-monsters

struct Solution {}

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut turns_to_reach = dist
            .into_iter()
            .zip(speed.into_iter())
            .map(|(a, b)| a / b + (a % b != 0) as i32)
            .collect::<Vec<_>>();
        turns_to_reach.sort();

        let n = turns_to_reach.len();
        turns_to_reach
            .into_iter()
            .enumerate()
            .position(|(turn, turns_to_reach)| turns_to_reach <= turn as i32)
            .unwrap_or(n) as i32
    }
}
