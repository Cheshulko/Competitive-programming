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

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![];
        let mut max = 0;
        for i in 0..n {
            let s = cin.next::<String>().into_bytes();
            max = max.max(s.len());
            a.push(s);
        }

        let mut ans = vec![vec![b'*'; n]; max];

        for i in 0..n {
            for j in 0..a[i].len() {
                ans[j][n - 1 - i] = a[i][j];
            }
        }

        for j in 0..max {
            let s = String::from_utf8(ans[j].clone()).unwrap();
            let s2 = s.trim_end_matches("*");
            println!("{s2}");
        }
    }
}
