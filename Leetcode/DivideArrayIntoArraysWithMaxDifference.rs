// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference

struct Solution {}

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n % 3 != 0 {
            return vec![];
        }

        nums.sort_unstable();
        nums.chunks(3)
            .map(|v| (v[2] - v[0] <= k).then_some(v.to_owned()))
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default()
    }
}
