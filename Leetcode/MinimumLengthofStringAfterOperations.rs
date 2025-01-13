// https://leetcode.com/problems/minimum-length-of-string-after-operations

struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        s.into_bytes()
            .into_iter()
            .fold(vec![0; 26], |mut v, c| {
                v[(c - b'a') as usize] += 1;
                v
            })
            .into_iter()
            .fold(0, |acc, cnt| acc + cnt.min(1 + (cnt & 1 != 1) as i32))
    }
}
