use core::num;
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

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; k];
        let mut aa = vec![false; n];
        let mut adj = vec![vec![]; n];
        let mut deg = vec![0; n];

        for i in 0..n - 1 {
            let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
            let x = x - 1;
            let y = y - 1;
            adj[x].push(y);
            adj[y].push(x);
            deg[x] += 1;
            deg[y] += 1;
        }

        for i in 0..k {
            a[i] = cin.next::<usize>();
            a[i] -= 1;
            aa[a[i]] = true;
        }

        let mut rem = vec![false; n];
        let mut q = VecDeque::<usize>::new();
        for i in 0..n {
            if deg[i] == 1 {
                q.push_back(i);
            }
        }

        while let Some(cur) = q.pop_front() {
            let ok = !aa[cur];

            if ok && deg[cur] == 1 {
                rem[cur] = true;

                for &to in adj[cur].iter() {
                    deg[to] -= 1;
                    if deg[to] == 1 {
                        q.push_back(to);
                    }
                }
            }
        }

        let mut cnt = 0;
        for i in 0..n {
            if rem[i] {
                cnt += 1;
            }
        }

        println!("{ans}", ans = n - cnt);
    }
}
