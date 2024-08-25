use core::num;
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
        let n = cin.next::<usize>();
        let mut a = vec![];
        for i in 0..n {
            a.push(cin.next::<i64>());
        }

        let mut ans = 0;
        // let mut
        let mut st = 0;
        for i in 0..n {
            while st != 0 && a[i] > 0 {
                if st == 0 || st == 1 {
                    a[i] -= 1;
                    ans += 1;
                } else {
                    a[i] -= 3;
                    ans += 1;
                }
                st += 1;
                st %= 3;
            }

            if a[i] > 0 {
                let c = a[i] / 5;
                ans += 3 * c;
                a[i] -= c * 5;

                while a[i] > 0 {
                    if st == 0 || st == 1 {
                        a[i] -= 1;
                        ans += 1;
                    } else {
                        a[i] -= 3;
                        ans += 1;
                    }
                    st += 1;
                    st %= 3;
                }
            }
        }

        println!("{ans}");
    }
}
