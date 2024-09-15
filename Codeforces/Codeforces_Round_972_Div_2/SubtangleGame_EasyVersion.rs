use std::cmp;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::i64;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
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

fn solve(
    i: usize,
    j: usize,
    k: usize,
    n: usize,
    m: usize,
    g: &Vec<Vec<usize>>,
    a: &Vec<usize>,
    dp: &mut Vec<Vec<Vec<Option<bool>>>>,
) -> bool {
    if k == a.len() {
        return false;
    }
    if let Some(ans) = dp[i][j][k] {
        return ans;
    }

    let mut win = false;

    if i + 1 <= n {
        win |= solve(i + 1, j, k, n, m, g, a, dp);
    }
    if j + 1 <= m {
        win |= solve(i, j + 1, k, n, m, g, a, dp)
    };
    if i + 1 <= n && j + 1 <= m && a[k] == g[i][j] {
        win |= !solve(i + 1, j + 1, k + 1, n, m, g, a, dp);
    }

    dp[i][j][k] = Some(win);

    win
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    // let _t = 1;
    for _ in 0.._t {
        let (l, n, m) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut a = vec![0; l];
        for i in 0..l {
            a[i] = cin.next::<usize>();
        }

        let mut g = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                g[i][j] = cin.next::<usize>();
            }
        }

        let mut dp = vec![vec![vec![None; l]; m + 1]; n + 1];
        for i in 0..=n {
            for k in 0..l {
                dp[i][m][k] = Some(false);
            }
        }
        for j in 0..=m {
            for k in 0..l {
                dp[n][j][k] = Some(false);
            }
        }
        for i in (0..n).rev() {
            dp[i][m - 1][l - 1] = Some(dp[i + 1][m - 1][l - 1].unwrap() || a[l - 1] == g[i][m - 1]);
        }
        for j in (0..m).rev() {
            dp[n - 1][j][l - 1] = Some(dp[n - 1][j + 1][l - 1].unwrap() || a[l - 1] == g[n - 1][j]);
        }

        solve(0, 0, 0, n, m, &g, &a, &mut dp);

        if dp[0][0][0].unwrap() {
            println!("T");
        } else {
            println!("N");
        }
    }
}
