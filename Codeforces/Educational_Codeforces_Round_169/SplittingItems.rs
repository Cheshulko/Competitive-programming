use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
        let (n, mut k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        a.sort_unstable();

        let mut sa = 0;
        let mut sb = 0;

        let mut ali = true;
        let mut pr = a[n - 1];
        for mut x in a.into_iter().rev() {
            if ali {
                sa += x;
                pr = x;
            } else {
                let us = (pr - x).min(k);
                k -= us;
                x += us;

                sb += x;
            }
            ali = !ali;
        }

        sa += k / 2;
        sb += k / 2;

        println!("{ans}", ans = sa - sb);
    }
}
