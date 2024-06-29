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

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut g = vec![0; n + 1];

        let mut c = 1;
        for i in 0..n {
            let x = cin.next::<usize>();
            if i % k == 0 {
                c = 1;
            }
            g[x] = c;
            c += 1;
        }

        let mut need = -1;
        let mut swap = 0;
        let mut can = true;

        let mut c = 1;
        for i in 1..=n {
            if (i - 1) % k == 0 {
                c = 1;
            }
            if g[i] != c {
                if swap == 0 {
                    swap = i;
                    need = c;
                } else if need == g[i] {
                    let t = g[swap];
                    g[swap] = g[i];
                    g[i] = t;
                    swap = n + 1;
                    need = -1;
                } else {
                    can = false;
                    break;
                }
            }
            c += 1;
        }

        if can {
            if swap == n + 1 {
                println!("1");
            } else {
                println!("0");
            }
        } else {
            println!("-1");
        }
    }
}
