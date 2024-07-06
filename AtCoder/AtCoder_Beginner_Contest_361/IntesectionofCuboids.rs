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
        let mut b = vec![];
        let mut c = vec![];
        for i in 0..2 {
            let x = cin.next::<i32>();
            a.push(x);
            let x = cin.next::<i32>();
            b.push(x);
            let x = cin.next::<i32>();
            c.push(x);
        }

        let mut a1 = vec![];
        let mut b1 = vec![];
        let mut c1 = vec![];
        for i in 0..2 {
            let x = cin.next::<i32>();
            a1.push(x);
            let x = cin.next::<i32>();
            b1.push(x);
            let x = cin.next::<i32>();
            c1.push(x);
        }

        let d1 = a[1].min(a1[1]) - a[0].max(a1[0]);
        let d2 = b[1].min(b1[1]) - b[0].max(b1[0]);
        let d3 = c[1].min(c1[1]) - c[0].max(c1[0]);

        if d1 > 0 && d2 > 0 && d3 > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
