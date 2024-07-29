use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
use std::usize;
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

    let _t = 1;
    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();

        let n = s.len();

        let mut prev = 0;
        let mut t = vec![0; n + 1];
        let mut cnt = 1;

        for i in 1..=n {
            if s[i - 1] == b'F' {
                if i != cnt {
                    t[i] = (t[prev] + 1).max(i - cnt);
                } else {
                    t[i] = 0;
                }

                cnt += 1;
                prev = i;
            } else {
                t[i] = t[i - 1];
            }
        }

        println!("{x}", x = t[n]);
    }
}
