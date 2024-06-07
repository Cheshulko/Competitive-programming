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
        let (_, mut k, s) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<String>(),
        );

        let mut ans = vec![];
        let mut cur = 0;
        let mut ones = 0;
        for (i, c) in s.char_indices() {
            if c == '0' {
                let need = i - cur;
                if k >= need {
                    k -= need;
                    ans.push('0');
                    cur += 1;

                    if k == 0 {
                        for _ in 0..ones {
                            ans.push('1');
                        }
                        ones = 0;
                    }
                } else if k > 0 {
                    let to_set = i - k;
                    assert!(ones >= to_set - cur);

                    for _ in 0..(to_set - cur) {
                        ans.push('1');
                        ones -= 1;
                    }
                    ans.push('0');
                    for _ in 0..ones {
                        ans.push('1');
                    }
                    ones = 0;
                    k = 0;
                } else {
                    ans.push('0');
                }
            } else {
                if k != 0 {
                    ones += 1;
                } else {
                    ans.push('1');
                }
            }
        }
        for _ in 0..ones {
            ans.push('1');
        }

        let ans = ans.into_iter().collect::<String>();
        println!("{ans}");
    }
}
