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
        let n = cin.next::<i128>();

        let ns = format!("{n}").into_bytes();
        let nsl = ns.len();

        let mut ans = vec![];

        for l in 1..=6 {
            let mut x = vec![];

            for i in 0..l {
                x.push(ns[i % nsl]);
            }

            let x = String::from_utf8(x).unwrap();
            let xv = x.parse::<i128>().unwrap();

            for a in 1..=10000 {
                let b = n * a - xv;

                if b >= 1 && b <= 10000.min(a * n) {
                    if nsl as i128 * a - b == l as i128 {
                        ans.push((a, b));
                    }
                }
            }
        }

        println!("{x}", x = ans.len());
        for (a, b) in ans.into_iter() {
            println!("{a} {b}");
        }
    }
}
