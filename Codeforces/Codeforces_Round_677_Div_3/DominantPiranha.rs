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
        let mut mn = usize::MAX;
        let mut mx = usize::MIN;
        let mut mx_i = 0;

        let n = cin.next::<usize>();

        let mut a = vec![];
        for i in 0..n {
            let x = cin.next::<usize>();
            a.push(x);
            if x < mn {
                mn = x;
            }
            if x > mx {
                mx = x;
            }
        }

        for i in 0..n {
            if a[i] == mx {
                if i > 0 && a[i - 1] < mx {
                    mx_i = i;
                }
                if i + 1 < n && a[i + 1] < mx {
                    mx_i = i;
                }
            }
        }

        if mx == mn {
            println!("-1");
        } else {
            mx_i += 1;
            println!("{mx_i}");
        }
    }
}
