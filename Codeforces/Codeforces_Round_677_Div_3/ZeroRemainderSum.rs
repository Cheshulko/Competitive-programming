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

    let _t = 1;
    for _ in 0.._t {
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut grid = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                grid[i][j] = cin.next::<usize>();
            }
        }

        let mut ans = vec![None; k + 1];
        ans[0] = Some(0);

        for i in 0..n {
            let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; m]; k + 1];
            dp[0][0] = Some(0);
            for j in 0..m {
                let mut temp: Vec<Vec<Option<usize>>> = vec![vec![None; m]; k + 1];
                for take in 1..=(m / 2).min(j + 1) {
                    for rem in 0..k {
                        if let Some(d) = dp[rem][take - 1] {
                            if let Some(o) = &mut temp[(rem + grid[i][j]) % k][take] {
                                *o = (*o).max(d + grid[i][j]);
                            } else {
                                temp[(rem + grid[i][j]) % k][take] = Some(d + grid[i][j]);
                            }
                        }
                    }
                }
                for take_ in 1..=(m / 2).min(j + 1) {
                    for rem_ in 0..k {
                        if let Some(x1) = temp[rem_][take_] {
                            if let Some(x2) = &mut dp[rem_][take_] {
                                *x2 = (*x2).max(x1);
                            } else {
                                dp[rem_][take_] = Some(x1);
                            }
                        }
                    }
                }
            }

            let mut temp = ans.clone();
            for rem1_ in 0..k {
                for rem2_ in 0..k {
                    for take_ in 0..=(m / 2) {
                        if let Some(x1) = ans[rem1_] {
                            if let Some(x2) = dp[rem2_][take_] {
                                if let Some(o) = &mut temp[(rem1_ + rem2_) % k] {
                                    *o = (*o).max(x1 + x2);
                                } else {
                                    temp[(rem1_ + rem2_) % k] = Some(x1 + x2);
                                }
                            }
                        }
                    }
                }
            }

            for rem1_ in 0..k {
                if let Some(x1) = &mut ans[rem1_] {
                    if let Some(x2) = temp[rem1_] {
                        *x1 = (*x1).max(x2);
                    }
                } else {
                    ans[rem1_] = temp[rem1_];
                }
            }
        }

        println!("{x}", x = ans[0].unwrap_or(0));
    }
}
