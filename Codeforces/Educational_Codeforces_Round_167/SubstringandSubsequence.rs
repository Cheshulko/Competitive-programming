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
    // let _t = 1;
    for _ in 0.._t {
        // let n = cin.next::<usize>();
        let a = cin.next::<String>().into_bytes();
        let b = cin.next::<String>().into_bytes();

        let mut ans = a.len() + b.len();

        for j in 0..b.len() {
            for i in 0..a.len() {
                if a[i] == b[j] {
                    let mut cnt = 1;
                    let mut jj = j + 1;
                    let mut ii = i + 1;

                    while ii < a.len() && jj < b.len() {
                        if a[ii] != b[jj] {
                            ii += 1;
                        } else {
                            cnt += 1;
                            ii += 1;
                            jj += 1;
                        }
                    }

                    ans = ans.min(a.len() + b.len() - cnt);
                }
            }
        }

        println!("{ans}");
    }
}
