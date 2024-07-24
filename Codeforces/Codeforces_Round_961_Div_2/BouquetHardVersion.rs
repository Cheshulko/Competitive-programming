use core::num;
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
        let (n, m) = (cin.next::<usize>(), cin.next::<i128>());

        let mut a = vec![0; n];
        let mut c = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<i128>();
        }
        for i in 0..n {
            c[i] = cin.next::<i128>();
        }

        let mut a = a
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<_>>();

        a.sort_unstable();

        let mut ans = 0;

        for i in 0..n {
            let can = (m / a[i].0).min(c[a[i].1]);
            ans = ans.max(can * a[i].0);
        }

        for v in a.windows(2) {
            let x = v[0];
            let y = v[1];

            if y.0 - x.0 > 1 {
                continue;
            }

            let mut can_y = (m / y.0).min(c[y.1]);
            let mut left = m - can_y * y.0;
            let need = (x.0 - left % x.0) % x.0;

            let pp = left / x.0;

            if pp < c[x.1] {
                let free = c[x.1] - pp;
                if need > 0 && need <= can_y && need + 1 <= free {
                    can_y -= need;
                    left += need * y.0;
                    assert!(left % x.0 == 0);
                }
                let can_x = (left / x.0).min(c[x.1]);

                ans = ans.max(can_x * x.0 + can_y * y.0);
            } else {
                let can_x = (left / x.0).min(c[x.1]);
                ans = ans.max(can_x * x.0 + can_y * y.0);
            }
        }

        println!("{ans}");
    }
}
