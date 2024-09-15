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

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let v = [b'a', b'e', b'i', b'o', b'u'];

        let ea = n / v.len();
        let mut left = n - ea * v.len();

        let mut ans = vec![];
        for k in 0..v.len() {
            for _ in 0..ea {
                ans.push(v[k]);
            }
            if left > 0 {
                left -= 1;
                ans.push(v[k]);
            }
        }

        let s = ans.into_iter().map(|x| x as char).collect::<String>();
        println!("{s}");
    }
}
