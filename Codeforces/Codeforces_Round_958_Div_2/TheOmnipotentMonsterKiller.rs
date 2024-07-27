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

fn dfs(cur: usize, p: i32, adj: &Vec<Vec<usize>>, dp: &mut Vec<Vec<u64>>) {
    for &to in adj[cur].iter() {
        if to as i32 == p {
            continue;
        }

        dfs(to, cur as i32, adj, dp);

        for r in 0..=22 {
            let mut child_min = u64::MAX;

            for r_child in 0..=22 {
                if r == r_child {
                    continue;
                }

                child_min = child_min.min(dp[r_child][to]);
            }

            dp[r][cur] += child_min;
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<u64>();
        }

        let mut adj = vec![vec![]; n];
        for _ in 0..(n - 1) {
            let x = cin.next::<usize>();
            let y = cin.next::<usize>();

            adj[x - 1].push(y - 1);
            adj[y - 1].push(x - 1);
        }

        let mut dp = vec![vec![0; n]; 23];
        for i in 0..n {
            for r in 0..=22 {
                dp[r][i] = (r + 1) as u64 * a[i];
            }
        }

        dfs(0, -1, &adj, &mut dp);

        let mut ans = u64::MAX;
        for r in 0..=22 {
            ans = ans.min(dp[r][0]);
        }

        println!("{ans}");
    }
}
