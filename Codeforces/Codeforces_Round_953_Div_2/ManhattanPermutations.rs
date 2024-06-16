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
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        if k % 2 == 1 {
            println!("No");
            continue;
        }

        let mut a = vec![];
        for i in 0..n {
            a.push(i + 1);
        }

        let mut i = 0;
        let mut j = 0;
        let mut cur = 0;

        let mut r = n - 1;
        let mut l = 0;

        while cur != k && l <= r {
            i = l;
            j = r;
            let need = k - cur;
            let can = 2 * (j - i);
            if need > can {
                a.swap(l, r);
                cur += can;
            } else {
                let d = need / 2;
                a.swap(r - d, r);
                cur = k;
            }

            if r == 0 {
                break;
            }
            l += 1;
            r -= 1;
        }

        if cur != k {
            println!("No")
        } else {
            println!("Yes");
            for x in a.iter() {
                print!("{x} ");
            }
            println!();
        }
    }
}
