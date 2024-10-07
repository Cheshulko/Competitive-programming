// https://leetcode.com/problems/minimum-string-length-after-removing-substrings

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut st = VecDeque::<char>::new();
        let mut it = s.chars();

        let check = |st: &mut VecDeque<char>| -> bool {
            let len = st.len();
            if len >= 2 && st[len - 2] == 'A' && st[len - 1] == 'B' {
                st.pop_back();
                st.pop_back();

                return true;
            }
            if len >= 2 && st[len - 2] == 'C' && st[len - 1] == 'D' {
                st.pop_back();
                st.pop_back();

                return true;
            }

            return false;
        };

        while let Some(c) = it.next() {
            st.push_back(c);

            while check(&mut st) {}
        }

        st.len() as i32
    }
}
