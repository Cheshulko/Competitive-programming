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
        let mut s = cin.next::<String>().into_bytes();
        let t = cin.next::<String>().into_bytes();

        let n = s.len();
        let m = t.len();

        if m > n {
            println!("NO");
            continue;
        }

        let mut pos = vec![vec![]; 26];
        let mut uq = vec![];
        for i in 0..n {
            if s[i] == b'?' {
                uq.push(i);
            } else {
                pos[(s[i] - b'a') as usize].push(i);
            }
        }

        let mut can = true;
        let mut cur = 0;
        for i in 0..m {
            let puq = uq.partition_point(|x| *x < cur);
            let la = pos[(t[i] - b'a') as usize].partition_point(|x| *x < cur);

            let mut p = usize::MAX;
            let mut up = usize::MAX;

            if puq < uq.len() && uq[puq] >= cur {
                p = p.min(uq[puq]);
            }
            if la < pos[(t[i] - b'a') as usize].len() && pos[(t[i] - b'a') as usize][la] >= cur {
                up = up.min(pos[(t[i] - b'a') as usize][la]);
            }

            if up != usize::MAX && up < p {
                cur = up + 1;
            } else if p != usize::MAX && p < up {
                s[p] = t[i];
                cur = p + 1;
            } else {
                can = false;
                break;
            }
        }

        if can {
            println!("YES");
            for i in 0..s.len() {
                if s[i] == b'?' {
                    s[i] = b'x';
                }
            }
            let ss = s.into_iter().map(|x| x as char).collect::<String>();
            println!("{ss}");
        } else {
            println!("NO");
        }
    }
}
