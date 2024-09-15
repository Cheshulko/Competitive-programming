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

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, mut m, q) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut b = vec![0; m];
        for i in 0..m {
            b[i] = cin.next::<usize>();
            b[i] -= 1;
        }

        if m == 1 {
            b.push(b[0]);
            m += 1;
        }

        b.sort_unstable();

        let mut d = vec![0; q];
        for i in 0..q {
            d[i] = cin.next::<usize>();
            d[i] -= 1;
        }

        for i in 0..q {
            let mut b2 = vec![];

            if d[i] < b[0] {
                b2 = vec![b[0], b[1]];
            } else if d[i] > b[m - 1] {
                b2 = vec![b[m - 2], b[m - 1]];
            } else {
                let p = b.partition_point(|&x| x < d[i]);
                b2 = vec![b[p - 1], b[p]];
            }

            let b = b2;

            if b[0] == d[i] || b[1] == d[i] {
                println!("0");
                continue;
            }

            if b[0] < d[i] && d[i] < b[1] {
                let ans = (b[1] - b[0]) / 2;
                println!("{ans}");
            } else {
                if d[i] < b[0] {
                    let ans = d[i] + (b[0] - d[i]);
                    println!("{ans}");
                } else {
                    let ans = n - d[i] - 1 + (d[i] - b[1]);
                    println!("{ans}");
                }
            }
        }
    }
}
