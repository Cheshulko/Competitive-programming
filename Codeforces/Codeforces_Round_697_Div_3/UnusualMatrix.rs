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
        let n = cin.next::<usize>();

        let mut a = vec![vec![0; n]; n];
        for i in 0..n {
            let s = cin.next::<String>().into_bytes();
            for (j, c) in s.into_iter().enumerate() {
                a[i][j] = (c == b'1') as usize;
            }
        }

        let _ = cin.next::<String>();

        let mut b = vec![vec![0; n]; n];
        for i in 0..n {
            let s = cin.next::<String>().into_bytes();
            for (j, c) in s.into_iter().enumerate() {
                b[i][j] = (c == b'1') as usize;
            }
        }

        let mut c = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                c[i][j] = a[i][j] ^ b[i][j];
            }
        }

        let mut cols = vec![0; n];

        let mut ans = false;
        {
            let mut can = true;
            for j in 0..n {
                cols[j] = c[0][j];
            }
            for i in 1..n {
                let mut eq = 0;
                for j in 0..n {
                    eq += (cols[j] == c[i][j]) as usize;
                }
                if (eq == n) || (eq == 0) {
                } else {
                    can = false;
                    break;
                }
            }
            ans |= can;
        }
        {
            let mut can = true;
            for j in 0..n {
                cols[j] = 1 ^ c[0][j];
            }
            for i in 1..n {
                let mut eq = 0;
                for j in 0..n {
                    eq += (cols[j] == c[i][j]) as usize;
                }
                if (eq == n) || (eq == 0) {
                } else {
                    can = false;
                    break;
                }
            }
            ans |= can;
        }

        if ans {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
