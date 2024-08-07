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

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<i64>();
        }

        let mut d = vec![0; m];
        for i in 0..m {
            d[i] = cin.next::<i64>();
        }
        d.sort_unstable();

        let mut f = vec![0; k];
        for i in 0..k {
            f[i] = cin.next::<i64>();
        }
        f.sort_unstable();

        let mut cur_diff = 0;
        let mut cur_diff_i = 0;
        let mut cur_diff_cnt = 0;
        for i in 0..n - 1 {
            if a[i + 1] - a[i] > cur_diff {
                cur_diff = a[i + 1] - a[i];
                cur_diff_cnt = 1;
                cur_diff_i = i;
            } else if a[i + 1] - a[i] == cur_diff {
                cur_diff_cnt += 1;
            }
        }

        if cur_diff_cnt > 1 {
            println!("{cur_diff}");
            continue;
        }

        let mut new_diff = cur_diff;
        let mid = (a[cur_diff_i + 1] + a[cur_diff_i]) >> 1;
        for i in 0..m {
            let need = mid - d[i];
            let pos = f.partition_point(|x| x < &need);
            if pos < f.len() {
                let v = f[pos] + d[i];
                new_diff = new_diff.min((v - a[cur_diff_i]).max(a[cur_diff_i + 1] - v));
            }
            if pos > 0 {
                let v = f[pos - 1] + d[i];
                new_diff = new_diff.min((v - a[cur_diff_i]).max(a[cur_diff_i + 1] - v));
            }
        }

        let mut diff = i64::MIN;
        for i in 0..n - 1 {
            if a[i + 1] - a[i] == cur_diff {
                diff = diff.max(new_diff);
            } else {
                diff = diff.max(a[i + 1] - a[i]);
            }
        }

        println!("{diff}");
    }
}
