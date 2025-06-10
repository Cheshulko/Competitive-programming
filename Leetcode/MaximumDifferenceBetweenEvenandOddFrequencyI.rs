// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i

struct Solution {}

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let freq = s.into_bytes().into_iter().fold(vec![0; 26], |mut v, c| {
            v[(c - b'a') as usize] += 1;
            v
        });

        let mut odd = 0;
        let mut even = i32::MAX;

        for c in 0..26 {
            if freq[c] == 0 {
                continue;
            }
            if freq[c] & 1 == 1 {
                odd = odd.max(freq[c]);
            } else {
                even = even.min(freq[c]);
            }
        }

        odd - even
    }
}
