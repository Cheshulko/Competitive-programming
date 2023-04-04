// https://leetcode.com/problems/optimal-partition-of-string

struct Solution {}

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut used = vec![false; 30];
        let mut ans = 0;
        for c in s.chars() {
            let ascii = (c as u32 - 97) as usize;
            if used[ascii] {
                ans += 1;
                used = vec![false; 30];
            }
            used[ascii] = true;
        }
        ans + 1
    }
}
