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

        let mut a = vec![0; n + 1];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut ans = 0;
        let mut cut = vec![0; n + 1];

        for i in 0..n {
            if a[i] == 0 {
                continue;
            }
            if cut[i] > 0 {
                let mut right = 0;
                if a[i] > cut[i] + 1 {
                    right = a[i] - (cut[i] + 1);
                }

                let left = cut[i] - 1;

                if right > 0 && left > 0 {
                    ans += 1;
                } else {
                    if right > 0 {
                        ans += 1;
                        if right <= 2 {
                            cut[i + 1] = cut[i] + 2;
                        }
                    }
                    if left > 0 {
                        ans += 1;
                        if left <= 2 {
                            cut[i + 1] = 1;
                        }
                    }
                }
            } else {
                ans += 1;

                if a[i] <= 2 {
                    cut[i + 1] = 1;
                }
            }
        }
        println!("{ans}");
    }
}
