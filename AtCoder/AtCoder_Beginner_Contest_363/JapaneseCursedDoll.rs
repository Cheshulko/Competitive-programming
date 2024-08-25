use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
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
        let (n, t, p) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut l = vec![0; n];
        for i in 0..n {
            l[i] = cin.next::<usize>();
        }

        let mut ans = 0;

        for i in 0..1000 {
            let mut cnt = 0;
            for k in 0..n {
                if l[k] >= t {
                    cnt += 1;
                }
            }
            if cnt >= p {
                ans = i;
                break;
            }

            for k in 0..n {
                l[k] += 1;
            }
        }

        println!("{ans}");
    }
}
