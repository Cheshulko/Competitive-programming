use core::num;
use std::cmp::*;
use std::collections::*;
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

    const D: usize = 2_000_000;
    const MAX: usize = 2 * D + 1;

    let _t = 1;
    for _ in 0.._t {
        let (n, dist) = (cin.next::<usize>(), cin.next::<usize>());

        let mut cnt_x = vec![0; MAX];
        let mut cnt_y = vec![0; MAX];

        let mut xs = vec![0; n];
        let mut ys = vec![0; n];

        for i in 0..n {
            xs[i] = (cin.next::<i64>() + D as i64) as usize;
            ys[i] = (cin.next::<i64>() + D as i64) as usize;
            cnt_x[xs[i]] += 1;
            cnt_y[ys[i]] += 1;
        }
        xs.sort_unstable();
        ys.sort_unstable();

        let mut dist_y = vec![0; MAX];
        let mut min_y = usize::MAX;
        let mut min_y_ind = 0;
        {
            let mut d = 0;
            for i in 0..n {
                d += ys[i].abs_diff(0);
            }
            dist_y[0] = d;
            min_y = d;
            min_y_ind = 0;

            let mut have = n;
            let mut seen = 0;
            for i in 1..MAX {
                d -= have;
                d += seen;
                have -= cnt_y[i];
                seen += cnt_y[i];
                dist_y[i] = d;

                if d < min_y {
                    min_y = d;
                    min_y_ind = i;
                }
            }
        }

        let p1 = &dist_y[..min_y_ind];
        let p2 = &dist_y[min_y_ind..];

        let mut ans = 0;
        {
            let mut d = 0;
            for i in 0..n {
                d += xs[i].abs_diff(0);
            }
            if d == dist {
                ans += 1;
            }

            let mut have = n;
            let mut seen = 0;
            for i in 1..MAX {
                d -= have;
                d += seen;
                have -= cnt_x[i];
                seen += cnt_x[i];

                if d <= dist {
                    let have = dist - d;

                    let pos_1 = p1.len() - p1.partition_point(|x| x > &have);
                    let pos_2 = p2.partition_point(|x| x <= &have);

                    ans += pos_1 + pos_2;
                }
            }

            println!("{ans}");
        }
    }
}
