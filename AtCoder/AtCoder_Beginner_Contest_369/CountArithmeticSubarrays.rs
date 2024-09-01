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
        for i in 0..n {
            a[i] = cin.next::<i64>();
        }

        let mut i = 0;

        let mut ans = 0;
        while i < n {
            if i + 1 >= n {
                break;
            }

            let d = a[i + 1] - a[i];

            let mut cc = 1;
            while i + 1 < n && a[i + 1] - a[i] == d {
                i += 1;
                cc += 1;
            }

            for x in 2..=cc {
                ans += cc + 1 - x;
            }
        }

        ans += n;
        println!("{ans}");
    }
}
