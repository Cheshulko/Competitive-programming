// https://leetcode.com/problems/number-of-ways-to-split-array

struct Solution {}

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let suf = nums.iter().map(|&x| x as i64).sum::<i64>();
        let n = nums.len();

        nums.into_iter()
            .take(n - 1)
            .scan((0, 0, suf), |state, num| {
                state.1 += num as i64;
                state.2 -= num as i64;
                state.0 += (state.1 >= state.2) as i32;

                Some(*state)
            })
            .last()
            .unwrap()
            .0
    }
}
