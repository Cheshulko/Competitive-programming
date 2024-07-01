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

        let mut a = vec![(0, 0); n + 1];
        let mut w = vec![(0, 0, 0); n + 1];

        let mut itob = vec![0; n + 1];

        let mut b = vec![0; n + 1];
        for i in 0..n {
            let x = cin.next::<usize>();
            a[i] = (x, i);
            itob[i] = x;
            b[x] += 1;
        }

        for i in 0..n {
            w[i] = (cin.next::<usize>(), a[i].0, i);
        }

        w.sort_unstable();

        let mut ans = 0;
        let mut bi = 1;
        for &(ww, in_b, i) in w.iter() {
            if b[in_b] > 1 {
                while b[bi] != 0 {
                    bi += 1;
                }
                b[in_b] -= 1;
                ans += ww;
                b[bi] += 1;
            }
        }

        println!("{ans}");
    }
}
