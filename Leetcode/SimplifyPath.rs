// https://leetcode.com/problems/simplify-path

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut s: VecDeque<String> = VecDeque::new();
        path.split('/')
            .filter(|x| !x.is_empty())
            .for_each(|x| match x {
                ".." => {
                    s.pop_back();
                }
                "." => {}
                _ => {
                    s.push_back(format!("/{}", x));
                }
            });
        if s.is_empty() {
            "/".to_string()
        } else {
            s.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join("")
        }
    }
}
