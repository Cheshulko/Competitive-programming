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

        let mut a = vec![0; n];
        let mut need = vec![0];
        let mut prev = 0;
        for i in 0..n {
            a[i] = cin.next::<i64>();
            if i != 0 && a[i] - prev < 0 {
                need.push((a[i] - prev).abs());
            }
            prev = a[i].max(prev);
        }

        need.sort_unstable();
        let mut ans = 0;
        for i in 1..need.len() {
            let x = need[i] - need[i - 1];
            if x > 0 {
                ans += x * (1 + (need.len() - i) as i64);
            }
        }
        println!("{ans}");
    }
}
