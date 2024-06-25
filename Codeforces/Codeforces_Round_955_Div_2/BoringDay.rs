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

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        if a < b {
            swap(&mut a, &mut b);
        }
        a %= b;
    }
    b
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, l, r) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut a = vec![0; n];
        let mut pref = vec![0; n + 1];
        for i in 0..n {
            a[i] = cin.next::<usize>();
            pref[i + 1] = pref[i] + a[i];
        }

        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;

        while i < n {
            let mut cur = pref[j] - pref[i];
            while j < n && cur < l {
                j += 1;
                cur = pref[j] - pref[i];
            }
            if cur >= l {
                if cur <= r {
                    ans += 1;
                    i = j;
                } else {
                    i += 1;
                }
            } else {
                break;
            }
        }

        println!("{ans}");
    }
}
