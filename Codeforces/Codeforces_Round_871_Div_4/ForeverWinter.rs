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

fn dfs(cur: usize, d: usize, t: usize, p: i32, adj: &Vec<Vec<usize>>) -> usize {
    if d == t {
        return 1;
    }
    let mut cnt = 0;
    for &to in adj[cur].iter() {
        if to as i32 != p {
            cnt += dfs(to, d + 1, t, cur as i32, adj);
        }
    }

    return cnt;
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut adj = vec![vec![]; n];
        let mut cnt = vec![0; n];
        for _ in 0..m {
            let x = cin.next::<usize>();
            let y = cin.next::<usize>();
            adj[x - 1].push(y - 1);
            adj[y - 1].push(x - 1);
            cnt[x - 1] += 1;
            cnt[y - 1] += 1;
        }

        for i in 0..n {
            if cnt[i] == 1 {
                let y = dfs(i, 0, 2, -1, &adj);
                let x = dfs(i, 0, 3, -1, &adj) + 1;

                println!("{x} {y}");
                break;
            }
        }
    }
}
