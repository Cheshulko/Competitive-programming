// https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end

struct Solution {}

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        s.into_bytes()
            .into_iter()
            .chain([b'1'].into_iter())
            .fold((0, 0, false), |(ones, ans, one_is_prev), cur| {
                if cur == b'1' {
                    (ones + 1, ans, true)
                } else {
                    if one_is_prev {
                        (ones, ans + ones, false)
                    } else {
                        (ones, ans, false)
                    }
                }
            })
            .1
    }
}
