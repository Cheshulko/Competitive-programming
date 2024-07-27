use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
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
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let a = cin.next::<String>().into_bytes();
        let b = cin.next::<String>().into_bytes();

        let mut pref_a = vec![vec![0; 30]; n + 1];
        let mut pref_b = vec![vec![0; 30]; n + 1];

        for i in 0..n {
            for c in 0..30 {
                pref_a[i + 1][c] = pref_a[i][c];
                pref_b[i + 1][c] = pref_b[i][c];
            }
            let ca = (a[i] - b'a') as usize;
            pref_a[i + 1][ca] += 1;

            let cb = (b[i] - b'a') as usize;
            pref_b[i + 1][cb] += 1;
        }

        for q_ in 0..q {
            let l = cin.next::<usize>();
            let r = cin.next::<usize>();

            let la = &pref_a[l - 1];
            let ra = &pref_a[r];

            let lb = &pref_b[l - 1];
            let rb = &pref_b[r];

            let mut ans = 0;
            let mut take = 0;
            for c in 0..30 {
                let mut ca = ra[c] - la[c];
                let mut cb = rb[c] - lb[c];

                cb += take;

                let d = ca - cb;
                take += d;

                if take > 0 {
                    ans += take;
                }
            }

            println!("{ans}");
        }
    }
}
