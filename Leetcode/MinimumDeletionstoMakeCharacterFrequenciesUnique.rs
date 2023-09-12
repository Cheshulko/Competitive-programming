// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let s = s.chars().map(|c| c as u8).collect::<Vec<_>>();

        let mut cnt_c = vec![0; 27];
        let mut cnt = BTreeSet::<i32>::new();

        for i in 0..=s.len() {
            cnt.insert(i as i32);
        }

        for c in s.into_iter() {
            cnt_c[(c - b'a') as usize] += 1;
        }

        let mut ans = 0;

        for cur in 0..27 {
            let cur_cnt = cnt_c[cur];

            if cur_cnt == 0 {
                continue;
            }

            if cnt.contains(&cur_cnt) {
                cnt.remove(&cur_cnt);
                continue;
            }

            let less = *cnt.range(..=cur_cnt).next_back().unwrap();
            ans += cur_cnt - less;

            if less != 0 {
                cnt.remove(&less);
            }
        }

        ans
    }
}
