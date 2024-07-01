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
        let n = cin.next::<usize>();

        let mut h = vec![0; n];
        for i in 0..n {
            h[i] = cin.next::<usize>();
        }

        let mut d = vec![0; n];
        let mut c = vec![0; n];
        c[n - 1] = h[n - 1];
        for i in (0..(n - 1)).rev() {
            if h[i] <= h[i + 1] {
                d[i] = d[i + 1] + h[i + 1] - h[i] + 1;
                c[i] = h[i];
            } else {
                let x = h[i] - h[i + 1];
                if x > d[i + 1] {
                    d[i] = 0;
                    c[i] = h[i];
                } else {
                    d[i] = d[i + 1] - x + 1;
                    c[i] = h[i + 1] + x;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(d[i] + c[i]);
        }

        println!("{ans}");
    }
}
