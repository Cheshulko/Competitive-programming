// https://leetcode.com/problems/maximum-score-from-removing-substrings

struct Solution {}

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        fn solve(s: Vec<char>, from: char, to: char, gain: i32) -> (Vec<char>, i32) {
            let mut rest = vec![];
            let mut ans = 0;

            for c2 in s {
                if let Some(&c1) = rest.last() {
                    if c1 == from && c2 == to {
                        ans += gain;
                        rest.pop();
                    } else {
                        rest.push(c2);
                    }
                } else {
                    rest.push(c2);
                }
            }

            (rest, ans)
        }

        let s = s.chars().collect();

        if x > y {
            let (rest, ans1) = solve(s, 'a', 'b', x);
            let (_, ans2) = solve(rest, 'b', 'a', y);

            ans1 + ans2
        } else {
            let (rest, ans1) = solve(s, 'b', 'a', y);
            let (_, ans2) = solve(rest, 'a', 'b', x);

            ans1 + ans2
        }
    }
}
