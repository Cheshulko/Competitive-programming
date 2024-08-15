use core::num;
use std::cmp::*;
use std::collections::*;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Rem;
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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut ab = vec![];
        for i in 0..n {
            let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
            ab.push((x, y));
        }

        let mut cost = vec![vec![usize::MAX; 201]; n];
        for (ind, &(x, y)) in ab.iter().enumerate() {
            for i in 0..=x {
                for j in 0..=y {
                    cost[ind][i + j] = cost[ind][i + j].min(i * y + j * x - i * j);
                }
            }
        }

        let mut dp = vec![vec![usize::MAX; 101]; n + 1];
        dp[0][0] = 0;
        for t in 1..101 {
            dp[0][t] = cost[0][t];
        }

        for i in 1..n {
            for t in 0..101 {
                for j in 0..=t {
                    if cost[i][t - j] != usize::MAX && dp[i - 1][j] != usize::MAX {
                        dp[i][t] = dp[i][t].min(dp[i - 1][j] + cost[i][t - j]);
                    }
                }
            }
        }

        if dp[n - 1][k] == usize::MAX {
            println!("-1");
        } else {
            println!("{ans}", ans = dp[n - 1][k]);
        }
    }
}
