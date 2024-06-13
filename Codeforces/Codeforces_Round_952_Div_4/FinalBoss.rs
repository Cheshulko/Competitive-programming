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
        let (h, n) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![];
        for _ in 0..n {
            let x = cin.next::<usize>();
            a.push(x);
        }

        let mut c = vec![];
        for _ in 0..n {
            let x = cin.next::<usize>();
            c.push(x);
        }

        let mut l = 0;
        let mut r = 2 * 1_000_000_000_000_usize;

        while r - l > 1 {
            let m = (l + r) / 2;

            let mut p = 0;
            for i in 0..n {
                p += ((m + c[i] - 1) / c[i]) * a[i];
            }

            if p >= h {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{r}");
    }
}
