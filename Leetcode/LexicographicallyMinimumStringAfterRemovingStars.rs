// https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars

struct Solution {}

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let s = s.into_bytes();

        let mut ans = s.clone();
        let mut indxs = vec![vec![]; 26];

        'out: for (i, c) in s.into_iter().enumerate() {
            if c == b'*' {
                ans[i] = b'.';

                for rem_c in 0..26 {
                    if let Some(rem_ind) = indxs[rem_c].pop() {
                        ans[rem_ind] = b'.';

                        continue 'out;
                    }
                }

                unreachable!()
            } else {
                indxs[(c - b'a') as usize].push(i);
            }
        }

        ans.into_iter()
            .filter(|&b| b != b'.')
            .map(|b| b as char)
            .collect()
    }
}
