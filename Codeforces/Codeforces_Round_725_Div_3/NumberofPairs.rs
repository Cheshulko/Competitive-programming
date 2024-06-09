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
        let (n, l, r) = (cin.next::<usize>(), cin.next::<i64>(), cin.next::<i64>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<i64>();
        }

        a.sort_unstable();

        let mut ans = 0;
        for i in 0..n {
            let lv = l - a[i];
            let rv = r - a[i];

            let lp = a.partition_point(|x| *x < lv);
            let rp = a.partition_point(|x| *x < (rv + 1));

            ans += rp - lp;
            if i >= lp && i < rp {
                ans -= 1;
            }
        }

        assert!(ans % 2 == 0);
        ans /= 2;
        println!("{ans}");
    }
}
