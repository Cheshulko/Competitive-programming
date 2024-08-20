use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

fn solve(x: usize, y: usize, par: &mut Vec<i32>, cin: &mut Cin) {
    println!("? {x} {y}", x = x + 1, y = y + 1);

    let _ = stdout().flush();

    let z = cin.next::<usize>();
    let z = z - 1;

    if z == x {
        par[y] = x as i32;
    } else {
        if par[z] == -1 {
            solve(x, z, par, cin);
        }
        if par[y] == -1 {
            solve(z, y, par, cin);
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut par = vec![-1; n];
        for i in (1..n).rev() {
            if par[i] == -1 {
                solve(0, i, &mut par, &mut cin);
            }
        }

        let mut ans = String::new();
        ans.extend("! ".chars());
        for i in 1..n {
            assert!(par[i] != -1);
            ans.extend(format!("{p} {i} ", i = i + 1, p = par[i] + 1).chars());
        }

        println!("{ans}");
        let _ = stdout().flush();
    }
}
