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
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (an, bn, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut boys = vec![0; an + 1];
        let mut girls = vec![0; bn + 1];

        let mut a = vec![0; k];
        for i in 0..k {
            a[i] = cin.next::<usize>();
        }

        let mut b = vec![0; k];
        for i in 0..k {
            b[i] = cin.next::<usize>();
        }

        for i in 0..k {
            boys[a[i]] += 1;
            girls[b[i]] += 1;
        }

        let mut ans = 0;
        for i in 0..k {
            let x = 1 + k - boys[a[i]] - girls[b[i]];
            ans += x;
        }

        ans /= 2;

        println!("{ans}");
    }
}
