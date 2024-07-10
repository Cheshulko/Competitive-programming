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

    let _t = 1;
    for _ in 0.._t {
        let mut have = vec![false; 500];

        let n = cin.next::<usize>();
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
            have[a[i]] = true;
        }

        let m = cin.next::<usize>();
        let mut b = vec![0; m];
        for i in 0..m {
            b[i] = cin.next::<usize>();
            have[b[i]] = true;
        }

        for i in 0..n {
            for j in 0..m {
                if !have[a[i] + b[j]] {
                    println!("{x} {y}", x = a[i], y = b[j]);
                    return;
                }
            }
        }
    }
}
