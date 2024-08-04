// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums

struct Solution {}

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let l = left as usize - 1;
        let r = right as usize - 1;
        let mut a = vec![];
        for i in 0..n {
            let mut s = 0;
            for j in i..n {
                s += nums[j];
                a.push(s);
            }
        }
        a.sort_unstable();
        let mut ans = 0;
        for i in l..=r {
            ans += a[i];
            ans %= 1_000_000_000 + 7;
        }

        ans
    }
}
