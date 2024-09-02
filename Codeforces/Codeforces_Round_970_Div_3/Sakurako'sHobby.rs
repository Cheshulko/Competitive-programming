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

fn dfs(
    cur: usize,
    p: &Vec<usize>,
    col: &Vec<usize>,
    ans: &mut Vec<i32>,
    used: &mut Vec<bool>,
    have: i32,
) -> i32 {
    if ans[cur] != -1 {
        return have + ans[cur];
    }

    used[cur] = true;

    let x = col[cur - 1] as i32;
    let x = if !used[p[cur - 1]] {
        dfs(p[cur - 1], p, col, ans, used, have + x)
    } else {
        x + have
    };

    ans[cur] = x as i32;
    return ans[cur];
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();
        let mut p = vec![0; n];
        for i in 0..n {
            p[i] = cin.next::<usize>();
        }
        let s = cin.next::<String>().into_bytes();
        let s = s
            .into_iter()
            .map(|x| (x == b'0') as usize)
            .collect::<Vec<_>>(); // 0 white

        let mut ans = vec![-1; n + 1];

        for i in 1..=n {
            if ans[i] == -1 {
                let mut used = vec![false; n + 1];
                ans[i] = dfs(i, &p, &s, &mut ans, &mut used, 0);
            }
        }

        for i in 1..=n {
            assert!(ans[i] != -1);
            print!("{xx} ", xx = ans[i]);
        }
        println!();
    }
}
