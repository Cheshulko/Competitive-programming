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

    const MOD: usize = 1_000_000_000 + 7;
    const N: usize = 250_000 + 5;

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();

        let mut dp = vec![vec![0; 2]; N];

        dp[1][0] = 1;
        dp[2][0] = 1;

        for i in 2..n {
            dp[i + 1][0] = (dp[i + 1][0] + dp[i][0] + dp[i][1]) % MOD;

            if s[i] == b'A' && s[i - 1] == b'B' && s[i - 2] == b'A' {
                dp[i + 1][1] = (dp[i + 1][1] + dp[i - 1][1] + dp[i - 1][0]) % MOD;
                dp[i + 1][0] = (dp[i + 1][0] + MOD - dp[i][1]) % MOD;
            }

            if s[i] == b'B' && s[i - 1] == b'A' && s[i - 2] == b'B' {
                dp[i + 1][1] = (dp[i + 1][1] + dp[i - 1][1] + dp[i - 1][0]) % MOD;
                dp[i + 1][0] = (dp[i + 1][0] + MOD - dp[i][1]) % MOD;
            }
        }

        println!("{x}", x = (dp[n][0] + dp[n][1]) % MOD);
    }
}
