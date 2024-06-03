// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence

struct Solution {}

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();

        let mut pos = vec![vec![]; 26];
        for (i, c) in s.into_iter().enumerate() {
            pos[(c - b'a') as usize].push(i);
        }

        let mut cur = 0;
        let mut cnt = 0;
        let n = t.len();
        for c in t.into_iter() {
            let c = (c - b'a') as usize;
            let ind = pos[c].partition_point(|x| x < &cur);
            if ind == pos[c].len() {
                break;
            } else {
                cnt += 1;
                cur = pos[c][ind] + 1;
            }
        }

        (n - cnt) as i32
    }
}
