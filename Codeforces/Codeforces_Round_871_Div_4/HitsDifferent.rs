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

    let mut dp: Vec<i64> = vec![0; 3 * 1000_000 + 2];
    dp[1] = 0;
    let mut s = 1;
    for row in 1..=2023_usize {
        for i in s..(s + row) {
            if i < s + row - 1 {
                dp[i + row + 1] -= dp[i - (row - 1)];
            }
            dp[i] += (i * i) as i64;
            dp[i + row] += dp[i];
            dp[i + row + 1] += dp[i];
        }

        s += row;
    }

    for _ in 0.._t {
        let n = cin.next::<usize>();

        println!("{ans}", ans = dp[n]);
    }
}
