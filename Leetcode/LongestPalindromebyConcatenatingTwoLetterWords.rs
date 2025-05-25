// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words

struct Solution {}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut have = [[0; 26]; 26];

        for word in words {
            let bs = word.into_bytes();
            let a = (bs[0] - b'a') as usize;
            let b = (bs[1] - b'a') as usize;

            have[a][b] += 1;
        }

        let mut ans = 0;
        for a in 0..26 {
            for b in 0..26 {
                let take = if a == b {
                    let take = have[a][b] / 2;
                    have[a][b] -= 2 * take;

                    take
                } else {
                    let take = have[a][b].min(have[b][a]);
                    have[a][b] -= take;
                    have[b][a] -= take;

                    take
                };

                ans += 4 * take;
            }
        }

        for a in 0..26 {
            if have[a][a] > 0 {
                ans += 2;

                break;
            }
        }

        ans
    }
}
