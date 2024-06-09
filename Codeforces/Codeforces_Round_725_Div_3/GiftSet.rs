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
        let (x, y, a, b) = (
            cin.next::<f64>(),
            cin.next::<f64>(),
            cin.next::<f64>(),
            cin.next::<f64>(),
        );

        let mut l = 0.;
        let mut r = (x / b).min(y / a);

        while r - l > 3.0 {
            let d = r - l;

            let m1 = l + d / 3.;
            let m2 = r - d / 3.;

            let g1 = (x - m1 * b) / a;
            let g11 = (y - m1 * a) / b;
            let g1 = g1.min(g11);

            let g2 = (x - m2 * b) / a;
            let g22 = (y - m2 * a) / b;
            let g2 = g2.min(g22);

            let a1 = m1 + g1;
            let a2 = m2 + g2;

            if a1 > a2 {
                r = m2;
            } else {
                l = m1;
            }
        }

        let l = l.floor() as usize;
        let r = r.ceil() as usize;
        let mut ans = 0;
        for i in l..=r {
            let ll = i as f64;

            let xx = x - i as f64 * b;
            let yy = y - i as f64 * a;
            if xx >= 0. && yy >= 0. {
                let cv = xx / a;
                let cu = yy / b;
                let rr = cv.min(cu);

                ans = ans.max((ll + rr) as usize);
            }
        }
        println!("{ans}");
    }
}
