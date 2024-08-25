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
        let mut a = vec![];

        for _ in 0..3 {
            let x = cin.next::<i128>();
            let y = cin.next::<i128>();
            a.push((x, y));
        }

        let mut ok = false;

        for i in 0..3 {
            for j in (i + 1)..3 {
                for k in 0..3 {
                    if k == i || k == j {
                        continue;
                    }

                    let v1 = (a[k].0 - a[i].0, a[k].1 - a[i].1);
                    let v2 = (a[k].0 - a[j].0, a[k].1 - a[j].1);

                    if v1.0 * v2.0 + v1.1 * v2.1 == 0 {
                        ok = true;
                    }
                }
            }
        }

        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
