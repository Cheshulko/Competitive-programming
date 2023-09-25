// https://leetcode.com/problems/find-the-difference

struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        ('a'..='z')
            .find(|c| {
                s.chars().filter(|c1| c1 == c).count() != t.chars().filter(|c2| c2 == c).count()
            })
            .unwrap()
    }
}
