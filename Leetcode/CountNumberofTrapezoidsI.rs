// https://leetcode.com/problems/count-number-of-trapezoids-i

struct Solution {}

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        const M: usize = 1_000_000_000 + 7;

        points
            .into_iter()
            .fold(HashMap::<i32, usize>::new(), |mut ys, p| {
                *ys.entry(p[1]).or_default() += 1;
                ys
            })
            .into_iter()
            .map(|(_, v)| (v - 1) * v / 2)
            .fold((0, 0), |(mut pref, mut res), cur| {
                res += pref * cur;
                res %= M;

                pref += cur;
                pref %= M;

                (pref, res)
            })
            .1 as i32
    }
}
