// https://leetcode.com/problems/maximum-odd-binary-number

struct Solution {}

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let ones = s.chars().filter(|c| c == &'1').count();
        let zeros = s.len() - ones;

        format!("{}{}1", "1".repeat(ones - 1), "0".repeat(zeros))
    }
}
