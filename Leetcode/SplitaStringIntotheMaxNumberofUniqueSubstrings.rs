// https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings

use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn solve<'input>(mem: &mut HashSet<&'input str>, s: &'input str) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut ans = 0;
        for i in 0..s.len() {
            if mem.contains(&s[..(i + 1)]) {
                continue;
            }

            mem.insert(&s[..(i + 1)]);
            ans = ans.max(1 + Solution::solve(mem, &s[(i + 1)..]));
            mem.remove(&s[..(i + 1)]);
        }

        ans
    }

    pub fn max_unique_split(s: String) -> i32 {
        let s = s.as_str();
        let mut mem = HashSet::<&str>::new();

        Solution::solve(&mut mem, &s)
    }
}
