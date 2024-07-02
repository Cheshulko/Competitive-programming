use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;

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
        let s = cin.next::<String>().into_bytes();
        let n = s.len();

        let mut pos = vec![vec![-1]; 26];
        for (i, c) in s.into_iter().enumerate() {
            pos[(c - b'a') as usize].push(i as i32);
        }

        let mut ans = n;
        for c in 0..26 {
            pos[c].push(n as i32);

            let mut cans = 0;
            for v in pos[c].windows(2) {
                let f = v[0];
                let s = v[1];

                cans = cans.max(((s - f) as f64).log2().ceil() as usize);
            }

            ans = ans.min(cans);
        }
        println!("{ans}");
    }
}
