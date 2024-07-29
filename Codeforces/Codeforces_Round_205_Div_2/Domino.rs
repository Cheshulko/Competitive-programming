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
        let n = cin.next::<usize>();

        let mut s1 = 0;
        let mut s2 = 0;
        let mut t01 = false;
        let mut t10 = false;
        for _ in 0..n {
            let x = cin.next::<usize>();
            let y = cin.next::<usize>();
            s1 += x;
            s2 += y;

            if x % 2 == 1 && y % 2 == 0 {
                t10 = true;
            }
            if x % 2 == 0 && y % 2 == 1 {
                t01 = true;
            }
        }

        let ans = match (s1 % 2, s2 % 2) {
            (0, 0) => 0,
            (1, 1) if t01 || t10 => 1,
            _ => -1,
        };

        println!("{ans}");
    }
}
