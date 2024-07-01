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

    let _t = 1;
    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();
        let t = cin.next::<String>().into_bytes();

        let n = s.len();

        let mut can = false;
        for i in 1..n {
            for c in 0..i {
                let mut ss = vec![];
                for k in (0..n).step_by(i) {
                    if k + c < s.len() {
                        ss.push(s[c + k]);
                    }
                }

                if ss.len() == t.len() {
                    let mut ok = true;
                    for k in 0..t.len() {
                        if ss[k] != t[k] {
                            ok = false;
                        }
                    }
                    if ok {
                        can = true;
                        break;
                    }
                }
            }
        }

        if can {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
