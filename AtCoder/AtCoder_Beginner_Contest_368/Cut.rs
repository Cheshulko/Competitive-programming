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
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());
        let mut x = VecDeque::<usize>::new();
        for i in 0..n {
            let d = cin.next::<usize>();
            x.push_back(d);
        }

        for i in 0..k {
            let y = x.pop_back().unwrap();
            x.push_front(y);
        }

        for xx in x.into_iter() {
            print!("{xx} ");
        }
        println!();
    }
}
