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
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        let mut aa = vec![0; n];

        for i in 0..n {
            a[i] = cin.next::<usize>();
        }
        for i in 0..n {
            aa[i] = cin.next::<usize>();
        }

        let mut l = 0;
        let mut r = n - 1;
        while l < n && a[l] == aa[l] {
            l += 1;
        }
        while r >= 1 && a[r] == aa[r] {
            r -= 1;
        }

        let mut ll = 0;
        let mut rr = n - 1;
        if l <= r {
            while l >= 1 && a[l - 1] == aa[l - 1] && aa[l - 1] <= aa[l] {
                l -= 1;
            }
            while r < n - 1 && a[r + 1] == aa[r + 1] && aa[r + 1] >= aa[r] {
                r += 1;
            }

            ll = l;
            rr = r;
        }

        ll += 1;
        rr += 1;

        println!("{ll} {rr}");
    }
}
