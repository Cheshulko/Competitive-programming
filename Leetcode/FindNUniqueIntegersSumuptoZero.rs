// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero

struct Solution {}

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut ans = vec![];

        for i in 1..=n / 2 {
            ans.push(i);
            ans.push(-i);
        }

        if n & 1 == 1 {
            ans.push(0);
        }

        ans
    }
}
