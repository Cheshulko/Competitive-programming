use core::num;
use std::cmp::*;
use std::collections::*;
use std::i128;
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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn dfs(cur: usize, adj: &Vec<Vec<usize>>, deg: &Vec<usize>, used: &mut Vec<bool>) -> bool {
    used[cur] = true;

    let mut ok = deg[cur] == 2;
    for &to in adj[cur].iter() {
        if !used[to] {
            ok &= dfs(to, adj, deg, used);
        }
    }
    ok
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut adj = vec![vec![]; n];
        let mut deg = vec![0; n];
        for _ in 0..m {
            let (v, u) = (cin.next::<usize>(), cin.next::<usize>());
            let v = v - 1;
            let u = u - 1;
            adj[v].push(u);
            adj[u].push(v);
            deg[v] += 1;
            deg[u] += 1;
        }

        let mut used = vec![false; n];
        let mut cnt = 0;
        for i in 0..n {
            if !used[i] {
                cnt += dfs(i, &adj, &deg, &mut used) as usize;
            }
        }

        println!("{cnt}");
    }
}
