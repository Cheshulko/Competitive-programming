// https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations

struct Solution {}

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;

        nums.sort_unstable();

        let mut seen = HashSet::new();
        let mut prev = i32::MIN;
        seen.insert(prev);

        let mut ans = 0;
        for num in nums {
            prev = prev.max(num - k);
            while seen.contains(&prev) {
                prev += 1;
            }
            if prev <= num + k {
                seen.insert(prev);
                ans += 1;
            }
        }

        ans
    }
}
