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
        let (n, x) = (cin.next::<usize>(), cin.next::<usize>());

        let mut ans = 0;
        for a in 1..x.min(n) {
            let max_c0 = x - a - 1;
            let max_c1 = (n - 1) / (a + 1);

            for b in 1..=max_c0.min(max_c1) {
                let c0 = x - a - b;
                let c1 = (n - a * b) / (a + b);
                let c = c0.min(c1);

                ans += c;
            }
        }

        println!(" {ans}");
    }
}
