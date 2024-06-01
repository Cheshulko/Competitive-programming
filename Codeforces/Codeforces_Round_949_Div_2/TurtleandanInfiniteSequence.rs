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
        let (n, m) = (cin.next::<i128>(), cin.next::<i128>());

        let mut ans = n;

        for b in 0..35 {
            let mut ti = i128::MAX;
            let bb = 1 << b;

            if n & bb > 0 {
                continue;
            }

            if bb < n {
                let prev = ((n - bb) >> b) << b;
                let prev = prev | (bb - 1);
                ti = ti.min(n - prev);
            }

            let next = ((n | bb) >> b) << b;
            ti = ti.min(next - n);

            if ti <= m {
                ans |= bb;
            }
        }

        println!("{ans}");
    }
}
