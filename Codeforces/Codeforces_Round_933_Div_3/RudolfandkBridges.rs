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

fn solve_row(arr: &[usize], d: usize) -> usize {
    let n = arr.len();
    let mut bh = BinaryHeap::<(Reverse<usize>, usize)>::new();
    let mut dp = vec![0; n + 1];
    bh.push((Reverse(1 + arr[0]), 0));
    for i in 1..n {
        let prev = i - 1 - d.min(i - 1);
        let mut best_ind = 0;
        let mut best_val = 0;
        while let Some((Reverse(top), ind)) = bh.pop() {
            if ind >= prev {
                best_ind = ind;
                best_val = top;
                break;
            }
        }

        dp[i] = 1 + arr[i] + best_val;
        bh.push((Reverse(best_val), best_ind));
        bh.push((Reverse(dp[i]), i));
    }

    dp[n - 1]
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m, k, d) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut a = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                a[i][j] = cin.next::<usize>();
            }
        }

        let mut rows = vec![0; n];
        for i in 0..n {
            rows[i] = solve_row(&a[i], d);
        }

        let mut ans = usize::MAX;
        let mut sum = 0;
        for i in 0..(k - 1) {
            sum += rows[i];
        }

        for i in (k - 1)..n {
            sum += rows[i];
            ans = ans.min(sum);

            sum -= rows[i + 1 - k];
        }

        println!("{ans}");
    }
}
