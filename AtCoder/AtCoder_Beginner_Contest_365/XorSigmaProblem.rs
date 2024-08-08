use core::num;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        let mut s = 0;
        for i in 0..n {
            a[i] = cin.next::<usize>();
            s += a[i];
        }

        let mut dp = vec![vec![vec![0; 2]; 30]; n + 1];
        let mut ans = 0;

        for i in 0..n {
            for b in 0..30 {
                if (1 << b) & a[i] > 0 {
                    dp[i + 1][b][1] += 1;
                    dp[i + 1][b][1] += dp[i][b][0];
                    dp[i + 1][b][0] += dp[i][b][1];
                } else {
                    dp[i + 1][b][0] += 1;
                    dp[i + 1][b][1] += dp[i][b][1];
                    dp[i + 1][b][0] += dp[i][b][0];
                }
            }
        }

        for i in 1..=n {
            for b in 0..30 {
                ans += dp[i][b][1] * (1 << b);
            }
        }
        ans -= s;

        println!("{ans}");
    }
}
