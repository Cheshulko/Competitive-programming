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

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let mut l = 0;
        let mut r = 1001;

        while r - l > 1 {
            let min = (l + l + r) / 3;
            let max = (l + r + r) / 3;

            println!("? {min} {max}");
            let _ = stdout().flush();

            let x = cin.next::<usize>();

            if x == min * max {
                l = max;
            } else if x == min * max + min {
                r = max;
                l = min;
            } else {
                r = min;
            }
        }

        println!("! {r}");
    }
}
