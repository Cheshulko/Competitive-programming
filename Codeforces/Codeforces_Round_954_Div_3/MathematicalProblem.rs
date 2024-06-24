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

fn solve(
    cnt: usize,
    i: usize,
    j: usize,
    s: &String,
    dp: &mut Vec<Vec<Option<i128>>>,
) -> Option<i128> {
    if dp[i][j].is_some() {
        return dp[i][j];
    }

    let ss = &s[i..=j];
    let n = ss.len();

    if cnt == 0 {
        if ss.len() == 0 {
            return None;
        }
        return Some(ss.parse::<i128>().unwrap());
    }

    if cnt + 2 > n {
        return None;
    }

    let mut ans = None;
    for ind in 0..n {
        for to_l in 0..=(cnt - 1) {
            let left = solve(to_l, i, i + ind, s, dp);
            let right = solve(cnt - 1 - to_l, i + ind + 1, j, s, dp);

            if left.is_none() || right.is_none() {
                continue;
            }

            let left = left.unwrap();
            let right = right.unwrap();
            let mn = left * right;
            let dod = left + right;

            if ans.is_none() {
                ans = Some(mn.min(dod));
            } else {
                let a = ans.unwrap();
                ans = Some(a.min(mn).min(dod));
            }
        }
    }

    dp[i][j] = ans;
    ans
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();
        let s = cin.next::<String>();

        if n <= 2 {
            let ans = s.parse::<i32>().unwrap();
            println!("{ans}");
            continue;
        }

        let mut dp = vec![vec![None; n + 1]; n + 1];
        let ans = solve(n - 2, 0, n - 1, &s, &mut dp);

        let ans = ans.unwrap();

        println!("{ans}");
    }
}
