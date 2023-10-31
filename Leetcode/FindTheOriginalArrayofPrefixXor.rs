// https://leetcode.com/problems/find-the-original-array-of-prefix-xor

struct Solution {}

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        pref.into_iter()
            .scan(0, |pref, x| {
                let y = *pref ^ x;
                *pref ^= y;
                Some(y)
            })
            .collect::<Vec<_>>()
    }
}
