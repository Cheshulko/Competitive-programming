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
    let t = cin.next::<usize>();

    'out: for _ in 0..t {
        let n = cin.next::<usize>();

        let mut a = vec![];
        let mut b = vec![];

        for i in 0..n {
            let x = cin.next::<i128>();
            a.push(x);
        }

        for i in 0..(n + 1) {
            let x = cin.next::<i128>();
            b.push(x);
        }

        let mut last = i128::MAX;

        let ll = *b.last().unwrap();
        for i in 0..n {
            let min = a[i].min(b[i]);
            let max = a[i].max(b[i]);

            if ll >= min && ll <= max {
                last = 0;
            } else {
                last = last.min(2 * min.abs_diff(ll) as i128);
                last = last.min(2 * max.abs_diff(ll) as i128);
                last = last.min(a[i].abs_diff(ll) as i128);
                last = last.min(b[i].abs_diff(ll) as i128);
            }
        }

        let mut ans = 0;

        for i in 0..n {
            ans += a[i].abs_diff(b[i]) as i128;
        }

        ans += last + 1;

        println!("{ans}");
    }
}
