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

fn go(a: Vec<usize>, n: usize) -> (Vec<usize>, usize) {
    let mut cnt = vec![0; n + 1];
    let mut b = vec![];

    let mut s = 0;
    let mut max = 0;
    for i in 0..n {
        cnt[a[i]] += 1;
        if cnt[a[i]] > 1 && a[i] > max {
            max = a[i];
        }
        s += max;
        b.push(max);
    }

    (b, s)
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    // let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        let mut s0 = 0;
        for i in 0..n {
            a[i] = cin.next::<usize>();
            s0 += a[i];
        }

        let (b, s) = go(a, n);
        s0 += s;
        let (b, s) = go(b, n);
        s0 += s;

        for i in 0..n {
            s0 += b[i] * (n - i - 1);
        }

        println!("{s0}");
    }
}
