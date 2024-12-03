// https://leetcode.com/problems/adding-spaces-to-a-string

struct Solution {}

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let s = s.chars();
        let mut ans = vec![];

        let mut i = 0;
        for (j, c) in s.into_iter().enumerate() {
            while i < spaces.len() && j == spaces[i] as usize {
                ans.push(' ');
                i += 1;
            }
            ans.push(c);
        }

        ans.into_iter().collect::<String>()
    }
}
