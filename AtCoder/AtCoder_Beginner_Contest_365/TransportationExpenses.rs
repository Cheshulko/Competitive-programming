use core::num;
use std::cmp::*;
use std::collections::*;
use std::i32;
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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = 0;
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
            s += a[i];
        }

        if s <= m {
            println!("infinite");
            continue;
        }

        let mut l = 0;
        let mut r = *a.iter().max().unwrap();

        while r - l > 1 {
            let mid = (l + r) / 2;
            let mut s = 0;
            for i in 0..n {
                s += a[i].min(mid);
            }

            if s <= m {
                l = mid;
            } else {
                r = mid;
            }
        }

        println!("{l}");
    }
}
