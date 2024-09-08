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

    let _t = 1;
    for _ in 0.._t {
        let (h, w, q) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut rows = vec![BTreeSet::<usize>::new(); h];
        let mut cols = vec![BTreeSet::<usize>::new(); w];

        for i in 0..h {
            for j in 0..w {
                rows[i].insert(j);
                cols[j].insert(i);
            }
        }

        let mut cnt = 0;
        for _ in 0..q {
            let (r, c) = (cin.next::<usize>(), cin.next::<usize>());
            let r = r - 1;
            let c = c - 1;

            if rows[r].contains(&c) {
                assert!(cols[c].contains(&r));
                cnt += 1;
                rows[r].remove(&c);
                cols[c].remove(&r);
            } else {
                if let Some(&less) = cols[c].range(..r).next_back() {
                    cnt += 1;
                    cols[c].remove(&less);
                    rows[less].remove(&c);
                }
                if let Some(&greater) = cols[c].range((r + 1)..).next() {
                    cnt += 1;
                    cols[c].remove(&greater);
                    rows[greater].remove(&c);
                }

                if let Some(&less) = rows[r].range(..c).next_back() {
                    cnt += 1;
                    rows[r].remove(&less);
                    cols[less].remove(&r);
                }
                if let Some(&greater) = rows[r].range((c + 1)..).next() {
                    cnt += 1;
                    rows[r].remove(&greater);
                    cols[greater].remove(&r);
                }
            }
        }

        println!("{x}", x = w * h - cnt);
    }
}
