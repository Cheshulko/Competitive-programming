use core::num;
use std::cmp::*;
use std::collections::*;
use std::i128;
use std::i32;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::rc;
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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    const MAX: usize = 1_000_000 + 1;
    let mut can = vec![false; MAX];
    can[0] = true;
    for i in 0..MAX {
        if can[i] && i + 2020 < MAX {
            can[i + 2020] = true;
        }
        if can[i] && i + 2021 < MAX {
            can[i + 2021] = true;
        }
    }

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();
        if can[n] {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
