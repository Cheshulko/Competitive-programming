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

fn dfs(cur: usize, adj: &Vec<Vec<(usize, u64)>>, p: i32, sum: u64, ans: &mut u64) -> u64 {
    let mut ch = vec![];

    for &(to, c) in adj[cur].iter() {
        if to as i32 == p {
            continue;
        }

        ch.push(c + dfs(to, adj, cur as i32, sum, ans));
    }

    let n = ch.len();
    if n == 0 {
        0
    } else {
        ch.sort_unstable();

        if n >= 2 {
            *ans = (*ans).min(sum - (ch[n - 1] + ch[n - 2]));
        }

        ch[n - 1]
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut adj = vec![vec![]; n];
        let mut s = 0;

        for _ in 0..(n - 1) {
            let x = cin.next::<usize>();
            let y = cin.next::<usize>();
            let c = cin.next::<u64>();

            adj[x - 1].push((y - 1, c));
            adj[y - 1].push((x - 1, c));

            s += c;
        }

        s *= 2;

        let mut ans = u64::MAX;
        for i in 0..n {
            if adj[i].len() == 1 {
                let x = dfs(i, &adj, -1, s, &mut ans);
                ans = ans.min(s - x);

                break;
            }
        }

        println!("{ans}");
    }
}
