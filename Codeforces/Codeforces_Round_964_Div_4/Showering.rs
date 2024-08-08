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
        let (mut n, s, m) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut lr = vec![(0, 0)];
        for i in 0..n {
            let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
            lr.push((x, y));
        }
        lr.push((m, m));
        lr.sort_unstable();

        n += 2;

        let mut can = false;
        for x in lr.windows(2) {
            let st = x[0].1;
            let en = x[1].0;
            if en - st >= s {
                can = true;
                break;
            }
        }

        if can {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
