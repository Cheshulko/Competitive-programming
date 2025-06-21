// https://leetcode.com/problems/minimum-deletions-to-make-string-k-special

struct Solution {}

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let n = word.len() as i32;

        let freq = word
            .into_bytes()
            .into_iter()
            .fold(vec![0; 26], |mut freq, b| {
                freq[(b - b'a') as usize] += 1;
                freq
            });

        let mut freq = freq.into_iter().filter(|&f| f > 0).collect::<Vec<_>>();
        let nf = freq.len();
        freq.sort_unstable();

        let mut ans = n;

        let mut i = 0;
        let mut j = 0;
        let mut cnt = 0;

        while j < nf {
            if freq[j] - freq[i] <= k {
                cnt += freq[j];
                j += 1;
            } else {
                let take = freq[i] + k;
                ans = ans.min(n - (cnt + (nf - j) as i32 * take));
                cnt -= freq[i];
                i += 1;
            }
        }

        ans.min(n - cnt)
    }
}
