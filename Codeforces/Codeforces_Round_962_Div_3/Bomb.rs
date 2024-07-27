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
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        let mut b = vec![0; n];

        for i in 0..n {
            a[i] = cin.next::<i64>();
        }
        for i in 0..n {
            b[i] = cin.next::<i64>();
        }

        let mut l = *a.iter().max().unwrap();
        let mut r = -1;
        let mut ans = 0;
        let mut used_k = 0;
        let k = k as i64;

        while l - r > 1 {
            let m = (l + r) / 2;

            let mut need_k = 0;
            let mut lans = 0;
            for i in 0..n {
                let dv = a[i] - m;
                if dv <= 0 {
                    continue;
                }
                let ki = dv / b[i] + (dv % b[i] > 0) as i64;
                lans += (a[i] + a[i] - (ki - 1) * b[i]) * ki / 2;
                need_k += ki;
            }

            if need_k <= k {
                l = m;
                ans = lans;
                used_k = need_k;
            } else {
                r = m;
            }
        }

        ans += l * (k - used_k);
        println!("{ans}");
    }
}
