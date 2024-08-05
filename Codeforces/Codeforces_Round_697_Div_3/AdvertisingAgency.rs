use core::num;
use std::cmp::*;
use std::collections::*;
use std::i128;
use std::i32;
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
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

mod cm_modular {
    pub fn gcd_extended(a: i64, m: i64) -> (i64, i64, i64) {
        if a == 0 {
            (m, 0, 1)
        } else {
            let (gcd, y, x) = gcd_extended(m % a, a);
            let z = x - (m / a) * y;
            (gcd, z, y)
        }
    }

    pub fn mod_inverse(b: i64, m: i64) -> i64 {
        let (gcd, x, _) = gcd_extended(b, m);
        if gcd != 1 {
            panic!("Inverse does not exist");
        } else {
            (x % m + m) % m
        }
    }

    pub fn modular_exponential(base: i64, mut power: i64, modulus: i64) -> i64 {
        if modulus == 1 {
            return 0;
        }

        let mut base = if power < 0 {
            mod_inverse(base, modulus)
        } else {
            base % modulus
        };

        let mut result = 1;
        power = power.abs();

        while power > 0 {
            if power & 1 == 1 {
                result = (result * base) % modulus;
            }
            power >>= 1;
            base = (base * base) % modulus;
        }
        result
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<i64>();
        }
        a.sort_unstable();
        a.reverse();

        let last = a[k - 1];

        let mut cnt_used = 0;
        for i in (0..k).rev() {
            if a[i] == last {
                cnt_used += 1;
            }
        }

        let mut cnt = 0;
        for i in 0..n {
            if a[i] == last {
                cnt += 1;
            }
        }

        const MOD: i64 = 1_000_000_000 + 7;
        let mut x = 1;
        for i in (cnt_used + 1)..=cnt {
            x = (x * i) % MOD;
        }

        let mut y = 1;
        for i in 1..=(cnt - cnt_used) {
            y = (y * i) % MOD;
        }

        let y = cm_modular::mod_inverse(y, MOD);
        let ans = (x * y) % MOD;

        println!("{ans}");
    }
}
