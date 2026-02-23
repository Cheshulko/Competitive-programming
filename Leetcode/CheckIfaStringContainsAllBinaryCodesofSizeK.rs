// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k

struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let s = s.into_bytes();
        let k = k as usize;

        if s.len() < k {
            return false;
        }

        let ma = 1 << k;
        let mut cur = 0;
        for i in 0..k - 1 {
            cur <<= 1;
            cur += (s[i] == b'1') as usize;
        }

        use std::collections::HashSet;
        let mut unique = HashSet::new();
        for i in (k - 1)..s.len() {
            cur <<= 1;
            cur += (s[i] == b'1') as usize;
            if cur >= ma {
                cur -= ma;
            }
            unique.insert(cur);
        }

        unique.len() == ma
    }
}
