use core::num;
use std::cmp::*;
use std::collections::*;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Rem;
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

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let w = cin.next::<usize>();
        let mut a = vec![0; w];
        for i in 0..w {
            a[i] = cin.next::<usize>();
        }
        a.sort_unstable();
        a.reverse();

        let mut mx = 0;
        let mut b = vec![1; m];
        let mut c = 1;
        for i in 0..=m - k {
            b[i] = c.min(k);
            b[m - i - 1] = c.min(k);
            mx = mx.max(b[i]);
            c += 1;
        }
        for i in (m - k + 1)..m {
            b[i] = mx;
        }
        for i in 0..m {
            b[m - 1 - i] = b[i];
        }

        let mut mx = 0;
        let mut bb = vec![1; n];
        let mut c = 1;
        for i in 0..=n - k {
            bb[i] = c.min(k);
            bb[n - i - 1] = c.min(k);
            mx = mx.max(bb[i]);
            c += 1;
        }
        for i in (n - k + 1)..n {
            bb[i] = mx;
        }
        for i in 0..n {
            bb[n - 1 - i] = bb[i];
        }

        let mut cnt = vec![];
        for &i in bb.iter() {
            for &j in b.iter() {
                cnt.push(i * j);
            }
        }
        cnt.sort_unstable();
        cnt.reverse();

        let mut ans = 0;
        for (i, x) in a.into_iter().enumerate() {
            ans += x * cnt[i];
        }
        println!("{ans}");
    }
}
