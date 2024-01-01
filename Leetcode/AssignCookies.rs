// https://leetcode.com/problems/assign-cookies

struct Solution {}

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut j = 0;
        let mut cnt = 0;

        for child in g.into_iter() {
            while j < s.len() && s[j] < child {
                j += 1;
            }
            if j == s.len() {
                break;
            }
            cnt += 1;
            j += 1;
        }

        cnt
    }
}
