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

fn dfs((i, j): (usize, usize), mat: &Vec<Vec<i64>>, used: &mut Vec<Vec<bool>>) -> i64 {
    let mut ans = mat[i][j];
    used[i][j] = true;

    for (i, j) in [(1, 0), (0, -1), (-1, 0), (0, 1), (1, 0)]
        .into_iter()
        .filter_map(|(di, dj)| {
            let to_i = (i as i32 + di) as usize;
            let to_j = (j as i32 + dj) as usize;
            (mat.get(to_i)?.get(to_j)? != &0).then_some((to_i, to_j))
        })
    {
        if !used[i][j] {
            ans += dfs((i, j), mat, used);
        }
    }
    ans
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut mat = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                mat[i][j] = cin.next::<i64>();
            }
        }

        let mut ans = 0;
        let mut used = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if !used[i][j] && mat[i][j] != 0 {
                    ans = ans.max(dfs((i, j), &mat, &mut used));
                }
            }
        }
        println!("{ans}");
    }
}
