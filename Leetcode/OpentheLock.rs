// https://leetcode.com/problems/open-the-lock

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        fn to_number(b: Vec<u8>) -> usize {
            let mut p = 1;
            let mut cur = 0;
            for b_ in b.into_iter() {
                let b_ = (b_ - b'0') as usize;
                cur += p * b_;
                p *= 10;
            }
            cur
        }

        fn increase(mut num: usize, dig: usize) -> usize {
            let mut ans = 0;
            let mut p = 1;
            for x in (1..=4).rev() {
                if dig == x {
                    ans += p * (((num % 10) + 1) % 10);
                } else {
                    ans += p * (num % 10);
                }
                p *= 10;
                num /= 10;
            }

            ans
        }

        fn decrease(mut num: usize, dig: usize) -> usize {
            let mut ans = 0;
            let mut p = 1;
            for x in (1..=4).rev() {
                if dig == x {
                    ans += p * (((num % 10) + 9) % 10);
                } else {
                    ans += p * (num % 10);
                }
                p *= 10;
                num /= 10;
            }

            ans
        }

        const MAX: usize = 10000;
        const START: usize = 0;

        let target = to_number(target.into_bytes());

        let mut deadends = deadends
            .into_iter()
            .map(|d| to_number(d.into_bytes()))
            .fold(vec![false; MAX], |mut v, x| {
                v[x as usize] = true;
                v
            });

        if deadends[START] {
            return -1;
        }

        let mut q = VecDeque::<(usize, usize)>::new();
        q.push_back((START, 0));
        deadends[START] = true;

        while let Some((cur, d)) = q.pop_front() {
            if cur == target {
                return d as i32;
            }
            for dig in 1..=4 {
                let to = increase(cur, dig);
                if !deadends[to] {
                    q.push_back((to, d + 1));
                    deadends[to] = true;
                }
                let to = decrease(cur, dig);
                if !deadends[to] {
                    q.push_back((to, d + 1));
                    deadends[to] = true;
                }
            }
        }

        return -1;
    }
}
