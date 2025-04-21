// https://leetcode.com/problems/count-the-hidden-sequences

struct Solution {}

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut pref = 0;
        let mut mi = 0;
        let mut ma = 0;

        for dif in differences {
            pref += dif as i64;
            mi = mi.min(pref);
            ma = ma.max(pref);
        }

        let d1 = upper - lower;
        let d2 = (ma - mi) as i32;

        (d1 - d2 + 1).max(0)
    }
}
