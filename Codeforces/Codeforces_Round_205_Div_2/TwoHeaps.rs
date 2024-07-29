use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    // let _t = cin.next::<usize>();
    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let a = (0..2 * n).map(|_| cin.next::<usize>()).collect::<Vec<_>>();
        let mut cnt = vec![0; 100];
        for &x in a.iter() {
            cnt[x] += 1;
        }

        let mut cnt = cnt
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<_>>();

        cnt.sort_unstable();

        let mut sp = vec![false; 100];
        for (c, ind) in cnt.iter_mut() {
            if *c > 1 {
                *c -= 2;
                sp[*ind] = true;
            }
        }

        let mut in_1 = 0;
        let mut in_2 = 0;
        let mut ans = vec![0; 2 * n];
        let mut used = vec![vec![false; 2]; 100];

        let mut uniq1 = 0;
        let mut uniq2 = 0;
        for (i, &x) in a.iter().enumerate() {
            if ans[i] == 0 && !sp[x] {
                if in_1 < in_2 {
                    ans[i] = 1;
                    in_1 += 1;
                    uniq1 += 1;
                } else {
                    ans[i] = 2;
                    in_2 += 1;
                    uniq2 += 1;
                }
            }
        }
        for (i, &x) in a.iter().enumerate() {
            if sp[x] {
                if !used[x][0] {
                    used[x][0] = true;
                    ans[i] = 1;
                    in_1 += 1;
                    uniq1 += 1;
                } else if !used[x][1] {
                    used[x][1] = true;
                    ans[i] = 2;
                    in_2 += 1;
                    uniq2 += 1;
                } else {
                    // skip
                }
            }
        }
        for (i, _) in a.into_iter().enumerate() {
            if ans[i] == 0 {
                if in_1 < in_2 {
                    ans[i] = 1;
                    in_1 += 1;
                } else {
                    ans[i] = 2;
                    in_2 += 1;
                }
            }
        }

        println!("{x}", x = uniq1 * uniq2);
        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
