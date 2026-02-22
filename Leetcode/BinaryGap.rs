// https://leetcode.com/problems/binary-gap

struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let w = (0..)
            .scan(n, |n, b| {
                (*n > 0).then(|| {
                    let p = *n & 1 == 1;
                    *n >>= 1;

                    p.then_some(b)
                })
            })
            .flatten()
            .collect::<Vec<_>>();

        w.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
    }
}
