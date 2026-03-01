// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers

struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        (n.into_bytes().into_iter().max().unwrap() - b'0') as i32
    }
}
