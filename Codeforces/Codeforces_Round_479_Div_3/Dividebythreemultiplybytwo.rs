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

mod cm {
    fn dfs_topological(
        cur: usize,
        edges: &Vec<Vec<usize>>,
        used: &mut Vec<bool>,
        ans: &mut Vec<usize>,
    ) {
        used[cur] = true;
        for to in &edges[cur] {
            if !used[*to] {
                dfs_topological(*to, edges, used, ans);
            }
        }
        ans.push(cur);
    }

    pub fn topological_sort(n: usize, edges: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut used = vec![false; n];

        for to in 0..n {
            if !used[to] {
                dfs_topological(to, edges, &mut used, &mut ans);
            }
        }

        ans
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut arr = vec![0; n];
        for i in 0..n {
            arr[i] = cin.next::<usize>();
        }

        let mut adj = vec![vec![]; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if arr[i] % 3 == 0 && arr[i] / 3 == arr[j] {
                    adj[i].push(j);
                }
                if arr[i] % 2 == 0 && arr[i] / 2 == arr[j] {
                    adj[j].push(i);
                }
            }
        }

        let tp = cm::topological_sort(n, &adj);
        for x in tp.into_iter().rev() {
            print!("{y} ", y = arr[x]);
        }
        println!();
    }
}
