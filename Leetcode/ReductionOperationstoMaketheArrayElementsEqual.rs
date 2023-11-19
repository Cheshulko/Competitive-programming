// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal

struct Solution {}

impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let last = *nums.last().unwrap();
        let n = nums.len();
        nums.into_iter()
            .enumerate()
            .rev()
            .fold((0, last), |(mut ans, mut prev), (ind, cur)| {
                if prev != cur {
                    prev = cur;
                    ans += n - ind - 1
                }
                (ans, prev)
            })
            .0 as i32
    }
}
