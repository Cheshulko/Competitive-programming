// https://leetcode.com/problems/maximum-score-from-removing-substrings

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let s = s.into_bytes().into_iter().collect::<VecDeque<_>>();

        let mut ans = 0;

        let (fi, se) = if x > y { (b'a', b'b') } else { (b'b', b'a') };
        let (mx, mn) = (x.max(y), x.min(y));

        let mut s1: Vec<u8> = vec![];
        for c in s.into_iter() {
            if let Some(&b) = s1.last() {
                if b == fi && c == se {
                    ans += mx;
                    s1.pop();
                } else {
                    s1.push(c);
                }
            } else {
                s1.push(c);
            }
        }

        let mut s2: Vec<u8> = vec![];
        for c in s1.into_iter() {
            if let Some(&b) = s2.last() {
                if b == se && c == fi {
                    ans += mn;
                    s2.pop();
                } else {
                    s2.push(c);
                }
            } else {
                s2.push(c);
            }
        }

        ans
    }
}
