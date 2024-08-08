use core::num;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
        let s = cin.next::<String>().into_bytes();

        let mut dp = vec![[0; 3]; n + 1];
        let mut ans = 0;
        for i in 1..=n {
            let cur = s[i - 1];

            let r = match cur {
                b'R' => {
                    dp[i][0] = dp[i - 1][1].max(dp[i - 1][2]) + 1;
                    dp[i][1] = dp[i - 1][0].max(dp[i - 1][2]);

                    dp[i][0].max(dp[i][1])
                }
                b'S' => {
                    dp[i][1] = dp[i - 1][0].max(dp[i - 1][2]) + 1;
                    dp[i][2] = dp[i - 1][0].max(dp[i - 1][1]);

                    dp[i][1].max(dp[i][2])
                }
                b'P' => {
                    dp[i][2] = dp[i - 1][0].max(dp[i - 1][1]) + 1;
                    dp[i][0] = dp[i - 1][1].max(dp[i - 1][2]);

                    dp[i][2].max(dp[i][0])
                }
                _ => unreachable!(),
            };

            if r < ans {
                break;
            } else {
                ans = r;
            }
        }

        println!("{ans}");
    }
}
