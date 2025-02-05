// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal

struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let ss = s1
            .into_bytes()
            .into_iter()
            .zip(s2.into_bytes().into_iter())
            .filter_map(|(a, b)| {
                let d = a as i32 - b as i32;
                (d != 0).then_some((d, b))
            })
            .collect::<Vec<_>>();

        ss.len() == 0 || (ss.len() == 2 && ss[0].0 + ss[1].0 == 0 && ss[0].1 != ss[1].1)
    }
}
