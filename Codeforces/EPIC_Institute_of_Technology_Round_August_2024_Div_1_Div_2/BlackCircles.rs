use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::rc;
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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut x = vec![0; n];
        let mut y = vec![0; n];

        for i in 0..n {
            x[i] = cin.next::<usize>();
            y[i] = cin.next::<usize>();
        }

        let (xs, ys) = (cin.next::<usize>(), cin.next::<usize>());
        let (xt, yt) = (cin.next::<usize>(), cin.next::<usize>());

        let dx = xs.abs_diff(xt);
        let dy = ys.abs_diff(yt);

        let d2 = dx * dx + dy * dy;

        let mut can = true;
        for i in 0..n {
            let dx = xt.abs_diff(x[i]);
            let dy = yt.abs_diff(y[i]);
            let d2c = dx * dx + dy * dy;

            if d2c <= d2 {
                can = false;
                break;
            }
        }

        if can {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
