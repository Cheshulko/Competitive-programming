// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits

struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        fn bits(mut x: i32) -> usize {
            let mut cnt = 0;
            while x > 0 {
                cnt += 1;
                x &= x - 1;
            }
            cnt
        }

        arr.sort_by(|a, b| match bits(*a).cmp(&bits(*b)) {
            std::cmp::Ordering::Equal => a.cmp(&b),
            r @ _ => r,
        });
        arr
    }
}
