// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k

struct Solution {}

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut ans = 0;
        let mut mi = i32::MAX;

        for x in nums.into_iter() {
            mi = mi.min(x);
            if x - mi > k {
                ans += 1;
                mi = x;
            }
        }

        ans + 1
    }
}
