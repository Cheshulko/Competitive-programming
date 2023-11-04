// https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank

struct Solution {}

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut ans = 0;
        for l in left.into_iter() {
            ans = ans.max(l);
        }
        for r in right.into_iter() {
            ans = ans.max(n - r);
        }
        ans
    }
}
