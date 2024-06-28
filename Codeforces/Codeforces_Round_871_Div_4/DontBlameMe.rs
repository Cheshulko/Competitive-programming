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

fn cnt(mut x: usize) -> usize {
    let mut c = 0;
    while x > 0 {
        c += x % 2;
        x /= 2;
    }
    c
}

fn main() {
    let mut cin = Cin::new();

    const MOD: usize = 1_000_000_000 + 7;

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut dp = vec![vec![0; 1 << 7]; n + 1];
        let mut ans = 0;
        for i in 0..n {
            dp[i + 1][a[i]] += 1;
            for b in 0..(1 << 7) {
                dp[i + 1][b] = (dp[i + 1][b] + dp[i][b]) % MOD;
                dp[i + 1][b & a[i]] = (dp[i + 1][b & a[i]] + dp[i][b]) % MOD;
            }
        }

        for b in 0..(1 << 7) {
            if cnt(b) == k {
                ans = (ans + dp[n][b]) % MOD;
            }
        }

        println!("{ans}");
    }
}
