// https://leetcode.com/problems/bag-of-tokens

struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }

        tokens.sort_unstable();

        let (mut l, mut r) = (0, tokens.len() - 1);
        let mut ans = 0;

        while l <= r {
            if power >= tokens[l] {
                power -= tokens[l];
                l += 1;
                ans += 1;
            } else {
                if l == r {
                    break;
                } else {
                    if ans > 0 {
                        power += tokens[r] - tokens[l];
                        r -= 1;
                        l += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        ans
    }
}
