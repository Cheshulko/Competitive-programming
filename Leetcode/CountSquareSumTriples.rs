// https://leetcode.com/problems/count-square-sum-triples

struct Solution {}

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut cnt = 0;
        for a in 3..=n {
            for b in 3..=n {
                for c in 3..=n {
                    cnt += (a * a + b * b == c * c) as i32;
                }
            }
        }

        cnt
    }
}
