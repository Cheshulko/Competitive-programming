use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    // let _t = cin.next::<usize>();
    let _t = 1;
    for _ in 0.._t {
        // let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();

        let n = s.len();
        let mut ins = vec![0; n + 1];
        let mut outs = vec![0; n + 1];

        for i in 0..n {
            if s[i] == b'0' {
                outs[i] += 1;
                ins[(i + 1) % n] += 1;
            } else {
                ins[i] += 1;
                outs[(i + 1) % n] += 1;
            }
        }

        let mut hlvec = vec![];
        for i in 0..n {
            if ins[i] == 0 {
                hlvec.push(i);
            }
            if outs[i] == 0 {
                hlvec.push(i);
            }
        }

        let mut gans = 0;

        for t in 0..2 {
            let mut ans = 0;
            let mut used = vec![0; n];
            for i in 0..hlvec.len() {
                let x0 = hlvec[i];
                let x1 = hlvec[(i + 1) % hlvec.len()];

                if (x1 + n - x0) % n > 1 {
                    if used[x0] == 2 || used[x1] == 2 {
                    } else {
                        used[x1] = 1;
                        used[x0] = 1;
                        ans += 1;
                    }
                } else {
                    if used[x0] == 1 {
                        if used[x1] == 0 {
                            used[x1] = 2;
                            used[hlvec[(i + 2) % hlvec.len()]] = 1;
                            ans += 1;
                        }
                    } else if used[x0] == 0 {
                        if i == 0 {
                            if t == 0 {
                                used[x0] = 2;
                                used[hlvec[(i + 1) % hlvec.len()]] = 1;
                                used[hlvec[(i + hlvec.len() - 1) % hlvec.len()]] = 1;
                                ans += 1;
                            } else {
                                used[x1] = 2;
                                used[x0] = 1;
                                used[hlvec[(i + 2) % hlvec.len()]] = 1;
                                ans += 1;
                            }
                        } else {
                            used[x0] = 2;
                            used[hlvec[(i + 1) % hlvec.len()]] = 1;
                            used[hlvec[(i + hlvec.len() - 1) % hlvec.len()]] = 1;
                            ans += 1;
                        }
                    }
                }
            }

            gans = gans.max(ans);
        }

        println!("{gans}");
    }
}
