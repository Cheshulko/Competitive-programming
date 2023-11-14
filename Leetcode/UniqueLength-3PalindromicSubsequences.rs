//https://leetcode.com/problems/unique-length-3-palindromic-subsequences

struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let s = s
            .chars()
            .map(|c| (c as u8 - b'a') as usize)
            .collect::<Vec<_>>();
        let mut pref = vec![vec![0; 26]; n + 1];
        let mut cnt = vec![vec![]; 26];
        for (ind, c) in s.iter().enumerate() {
            for c_cur in 0..26 {
                pref[ind + 1][c_cur] = pref[ind][c_cur];
            }
            pref[ind + 1][*c] += 1;
        }
        for (ind, c) in s.iter().enumerate() {
            cnt[*c].push(ind);
        }
        let mut ans = 0;
        for c in 0..26 {
            if cnt[c].len() == 0 {
                continue;
            }
            let left = cnt[c][0] + 1;
            let right = cnt[c][cnt[c].len() - 1];
            if let (Some(left), Some(right)) = (pref.get(left), pref.get(right)) {
                let mut cnt_cur = 0;
                for cc in 0..26 {
                    cnt_cur += (right[cc] - left[cc] > 0) as i32;
                }
                ans += cnt_cur;
            }
        }

        ans
    }
}
