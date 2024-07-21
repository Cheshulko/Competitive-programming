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
        let mut cnt = vec![0; n + 1];
        for i in 0..n {
            a[i] = cin.next::<usize>();
            cnt[a[i]] += 1;
        }
        let mx = *a.iter().max().unwrap();

        if cnt[mx] % 2 == 1 {
            println!("Yes");
        } else {
            if cnt[mx] == n {
                println!("No");
            } else {
                let mut can = false;
                for i in 0..=n {
                    if i == mx {
                        continue;
                    }

                    if cnt[i] > 0 && cnt[i] % 2 == 1 {
                        println!("Yes");
                        can = true;
                        break;
                    }
                }
                if !can {
                    println!("No");
                }
            }
        }
    }
}
