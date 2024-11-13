// https://leetcode.com/problems/count-the-number-of-fair-pairs

struct Solution {}

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();

        let mut ans = 0;
        for (i, x) in nums.iter().enumerate() {
            let lo = lower - x;
            let up = upper - x;

            let pl = &nums[(i + 1)..].partition_point(|&y| y < lo);
            let pu = &nums[(i + 1)..].partition_point(|&y| y <= up);

            ans += (pu - pl) as i64;
        }

        ans
    }
}
