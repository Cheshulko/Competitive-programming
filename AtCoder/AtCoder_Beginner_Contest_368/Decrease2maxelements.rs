use core::num;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::i64;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
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

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();
        let mut a = vec![];
        for i in 0..n {
            let d = cin.next::<usize>();
            a.push(d);
        }

        let mut ans = 0;
        loop {
            a.sort();
            a.reverse();
            if a[1] == 0 {
                break;
            } else {
                let x = a[0].min(a[1]);
                a[0] -= x;
                a[1] -= x;
                ans += x;
            }
        }

        println!("{ans}");
    }
}
