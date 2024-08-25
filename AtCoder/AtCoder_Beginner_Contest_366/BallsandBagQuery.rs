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

        let mut hs = HashMap::<usize, usize>::new();
        for i in 0..n {
            let t = cin.next::<usize>();

            if t == 1 {
                let x = cin.next::<usize>();
                *hs.entry(x).or_default() += 1;
            } else if t == 2 {
                let x = cin.next::<usize>();
                let y = *hs.get(&x).unwrap();

                if y == 1 {
                    hs.remove(&x);
                } else {
                    hs.insert(x, y - 1);
                }
            } else {
                println!("{x}", x = hs.len());
            }
        }
    }
}
