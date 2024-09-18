// https://leetcode.com/problems/largest-number

struct Solution {}

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|&a, &b| {
            let ab = format!("{a}{b}");
            let ba = format!("{b}{a}");

            ba.cmp(&ab)
        });

        let s = nums.into_iter().fold(String::new(), |mut s, n| {
            s.push_str(&format!("{n}"));
            s
        });
        let s = s.trim_start_matches('0');

        if s.is_empty() {
            String::from("0")
        } else {
            s.to_owned()
        }
    }
}
