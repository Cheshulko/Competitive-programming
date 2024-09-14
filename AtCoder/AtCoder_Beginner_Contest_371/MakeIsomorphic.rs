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

fn dfs(
    n: usize,
    cur: usize,
    used: &mut Vec<usize>,
    g: &Vec<Vec<bool>>,
    h: &Vec<Vec<bool>>,
    a: &Vec<Vec<usize>>,
    ans: &mut usize,
) {
    if cur == n {
        let mut s = 0;
        for i in 0..n {
            let to = used[i] - 1;

            for j in 0..n {
                let toj = used[j] - 1;

                if g[i][j] != h[to][toj] {
                    s += a[to][toj];
                }
            }
        }

        *ans = (*ans).min(s);

        return;
    }

    for i in 0..n {
        if used[i] == 0 {
            used[i] = cur + 1;
            dfs(n, cur + 1, used, g, h, a, ans);
            used[i] = 0;
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut g = vec![vec![false; n]; n];
        let mut h = vec![vec![false; n]; n];

        let mut a = vec![vec![usize::MAX; n]; n];

        let m = cin.next::<usize>();
        for i in 0..m {
            let (u, v) = (cin.next::<usize>(), cin.next::<usize>());
            g[u - 1][v - 1] = true;
            g[v - 1][u - 1] = true;
        }

        let m = cin.next::<usize>();
        for i in 0..m {
            let (u, v) = (cin.next::<usize>(), cin.next::<usize>());
            h[u - 1][v - 1] = true;
            h[v - 1][u - 1] = true;
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let x = cin.next::<usize>();
                a[i][j] = x;
                a[j][i] = x;
            }
        }

        for i in 0..n {
            a[i][i] = 0;
        }

        let mut used = vec![0; n];
        let mut ans = usize::MAX;

        dfs(n, 0, &mut used, &g, &h, &a, &mut ans);
        println!("{ans}", ans = ans / 2);
    }
}
