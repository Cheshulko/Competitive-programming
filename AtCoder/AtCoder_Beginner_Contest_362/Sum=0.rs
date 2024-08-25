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
        let n = cin.next::<usize>();

        let mut v = vec![];
        let mut ll = 0;
        let mut rr = 0;
        for _ in 0..n {
            let l = cin.next::<i64>();
            let r = cin.next::<i64>();

            v.push((l, r));

            ll += l;
            rr += r;
        }

        let mut ans = vec![];
        let mut s = 0;

        if ll <= 0 && rr >= 0 {
            println!("Yes");

            for i in 0..n {
                s += v[i].0;
                ans.push(v[i].0);
            }

            for i in 0..n {
                if s < 0 {
                    let dv = v[i].1 - v[i].0;
                    let dv = s.abs().min(dv);
                    s += dv;
                    ans[i] += dv;
                }
            }

            assert!(s == 0);
            assert!(ans.iter().sum::<i64>() == 0);
            for x in ans.into_iter() {
                print!("{x} ");
            }
            println!();
        } else {
            println!("No");
        }
    }
}
