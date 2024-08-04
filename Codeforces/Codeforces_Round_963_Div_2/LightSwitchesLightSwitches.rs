use core::num;
use std::cmp::*;
use std::collections::*;
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

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 || b == b {
        return 0;
    }
    while a != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    b
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<i128>());

        let mut a = vec![0; n];

        for i in 0..n {
            a[i] = cin.next::<i128>();
        }
        a.sort_unstable();

        let mut l: i128 = 0;
        let mut r: i128 = 1_000_000_000_000_000;

        for i in 0..n {
            let d = (a[i] - r + (2 * k - 1)) / (2 * k);

            l += 2 * k * d;
            r += 2 * k * d;

            l = l.max(a[i]);
            r = r.min(a[i] + k - 1);
        }

        if l <= r {
            println!("{l}");
        } else {
            println!("-1");
        }
    }
}
