use core::num;
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

    let _t = cin.next::<usize>();
    // let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut ans = 0;
        let mut cnt: Vec<usize> = vec![0; n];
        let mut can = true;

        for i in 1..n {
            if a[i] == 1 {
                if a[i - 1] != 1 {
                    can = false;
                    break;
                }
            } else {
                if a[i] <= a[i - 1] {
                    let mut cc = 0;
                    let mut at = a[i];
                    while at < a[i - 1] {
                        at = at * at;
                        cc += 1;
                    }

                    cnt[i] = cnt[i - 1] + cc;
                } else {
                    // a[i - 1] < a[i]
                    let mut cnt_prev = cnt[i - 1];
                    let mut prev = a[i - 1];
                    while cnt_prev > 0 && prev < a[i] {
                        cnt_prev -= 1;
                        prev = prev * prev;
                    }

                    if prev > a[i] {
                        cnt_prev += 1;
                    }

                    cnt[i] = cnt_prev;
                }
            }
        }

        if can {
            for i in 0..n {
                ans += cnt[i];
            }
            println!("{ans}");
        } else {
            println!("-1");
        }
    }
}
