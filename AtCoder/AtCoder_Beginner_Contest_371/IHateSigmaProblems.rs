use std::cmp;
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

        let mut a = vec![0; n];
        let mut pos = vec![vec![0]; n + 1];
        for i in 0..n {
            a[i] = cin.next::<usize>();
            pos[a[i]].push(i + 1);
        }

        for i in 0..=n {
            if pos[i].len() > 0 {
                pos[i].push(n + 1);
            }
        }

        let mut ans: usize = 0;
        for i in 0..=n {
            if pos[i].len() > 0 {
                ans += n * (1 + n) / 2;

                for x in pos[i].windows(2) {
                    let d = x[1] - x[0] - 1;
                    ans -= d * (1 + d) / 2
                }
            }
        }

        println!("{ans}");
    }
}
