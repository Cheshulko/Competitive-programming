// https://leetcode.com/problems/is-subsequence/description

struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.chars().map(|c| c as u8).collect::<Vec<_>>();
        let t = t.chars().map(|c| c as u8).collect::<Vec<_>>();

        let mut x = -1;
        let mut y = -1;
        for c1 in s.into_iter() {
            for (ind, c2) in t.iter().enumerate() {
                let ind = ind as i32;
                if &c1 == c2 && ind > x {
                    y = ind;
                    break;
                }
            }
            if y == x {
                return false;
            }
            x = y;
        }

        true
    }
}
