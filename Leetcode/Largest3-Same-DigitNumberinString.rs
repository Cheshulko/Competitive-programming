// https://leetcode.com/problems/largest-3-same-digit-number-in-string

struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.as_bytes()
            .windows(3)
            .filter(|number| number[0] == number[1] && number[1] == number[2])
            .max_by(|x, y| x[0].cmp(&y[1]))
            .map(|x| std::str::from_utf8(x).unwrap())
            .unwrap_or("")
            .to_string()
    }
}
