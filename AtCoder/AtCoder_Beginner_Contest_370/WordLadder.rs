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
        let mut s = cin.next::<String>().into_bytes();
        let t = cin.next::<String>().into_bytes();
        let n = s.len();

        let mut worse = vec![];
        let mut better = vec![];
        for i in 0..n {
            if s[i] > t[i] {
                worse.push(i);
            } else if s[i] < t[i] {
                better.push(i);
            }
        }

        let mut ans = vec![];

        for i in 0..worse.len() {
            s[worse[i]] = t[worse[i]];
            ans.push(s.clone());
        }

        for i in (0..better.len()).rev() {
            s[better[i]] = t[better[i]];
            ans.push(s.clone());
        }

        println!("{l}", l = ans.len());
        for i in 0..ans.len() {
            println!(
                "{x}",
                x = ans[i]
                    .clone()
                    .into_iter()
                    .map(|v| v as char)
                    .collect::<String>()
            );
        }
    }
}
