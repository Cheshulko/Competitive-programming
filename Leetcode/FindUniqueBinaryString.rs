// https://leetcode.com/problems/find-unique-binary-string

struct Solution {}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut x = vec![false; (1 << 16) + 1];

        for num in &nums {
            let mut p = 1;
            let mut a = 0;
            for y in num.chars().rev() {
                a |= ((y == '1') as usize) * p;
                p <<= 1;
            }
            x[a] = true;
        }

        let ans = x.iter().position(|x| x == &false).unwrap();
        let y = format!("{:#018b}", ans);

        let y = y.chars().rev().collect::<String>();
        let y = y.chars().take(n).collect::<String>();
        y.chars().rev().collect::<String>()
    }
}
