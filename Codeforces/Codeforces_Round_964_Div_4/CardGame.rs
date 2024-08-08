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
        let mut a = vec![0; 2];
        let mut b = vec![0; 2];

        for i in 0..2 {
            a[i] = cin.next::<usize>();
        }

        for i in 0..2 {
            b[i] = cin.next::<usize>();
        }

        let mut ans = 0;
        for i in 0..2 {
            for j in 0..2 {
                if a[i] > b[j] && a[1 - i] > b[1 - j]
                    || a[i] > b[j] && a[1 - i] == b[1 - j]
                    || a[i] == b[j] && a[1 - i] > b[1 - j]
                {
                    ans += 1;
                }
            }
        }

        println!("{ans}");
    }
}
