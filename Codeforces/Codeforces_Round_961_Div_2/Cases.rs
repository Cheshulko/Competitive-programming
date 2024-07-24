use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
use std::vec;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, c, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let s = cin
            .next::<String>()
            .into_bytes()
            .into_iter()
            .map(|c| (c - b'A') as usize)
            .collect::<Vec<_>>();

        let mut cur_mask = 0;
        let mut cnt = vec![0; c];
        let mut bad = vec![false; 1 << c];

        bad[1 << s[n - 1]] = true;

        for i in 0..(k - 1) {
            cnt[s[i]] += 1;
            if cnt[s[i]] == 1 {
                cur_mask |= 1 << s[i];
            }
        }

        for i in (k - 1)..n {
            cnt[s[i]] += 1;
            if cnt[s[i]] == 1 {
                cur_mask |= 1 << s[i];
            }

            bad[cur_mask] = true;

            cnt[s[i + 1 - k]] -= 1;
            if cnt[s[i + 1 - k]] == 0 {
                cur_mask ^= 1 << s[i + 1 - k];
            }
        }

        for mask in 0..(1 << c) {
            if bad[mask] {
                for ch in 0..c {
                    if ((mask >> ch) & 1) == 0 {
                        bad[mask | (1 << ch)] = true;
                    }
                }
            }
        }

        let mut ans = c;
        for mask in 0..(1 << c) {
            if !bad[mask] {
                ans = ans.min(c - mask.count_ones() as usize);
            }
        }

        println!("{ans}");
    }
}
