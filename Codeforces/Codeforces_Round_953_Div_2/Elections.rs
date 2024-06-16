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
        let (n, c) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![];
        let mut pr_max = vec![0; n + 1];
        let mut pr_sum = vec![0; n + 1];
        let mut mx = 0;
        for i in 0..n {
            let x = cin.next::<usize>();
            a.push(x);

            pr_max[i + 1] = pr_max[i].max(x);
            pr_sum[i + 1] = pr_sum[i] + x;
            mx = mx.max(x);
        }

        let mut ans = vec![0; n];

        for i in 0..n {
            if i == 0 {
                if a[i] + c >= mx {
                    ans[i] = 0;
                } else {
                    ans[i] = 1;
                }
            } else {
                if a[i] == mx {
                    if pr_max[i] < mx && a[0] + c < mx {
                        ans[i] = 0;
                    } else {
                        ans[i] = i;
                    }
                } else {
                    if a[i] + pr_sum[i] + c >= mx {
                        ans[i] = i;
                    } else {
                        ans[i] = i + 1;
                    }
                }
            }
        }

        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
