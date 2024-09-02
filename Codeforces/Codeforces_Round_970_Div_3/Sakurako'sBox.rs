use std::cmp::*;
use std::collections::*;
use std::i32;
use std::i64;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
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

    const MOD: usize = 1_000_000_000 + 7;

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut suf = vec![0; n + 1];
        for i in (0..n).rev() {
            suf[i] = suf[i + 1] + a[i];
            suf[i] %= MOD;
        }

        let mut x1 = 0;
        for i in 0..n {
            x1 += a[i] * suf[i + 1];
            x1 %= MOD;
        }

        x1 = x1 * 2;
        x1 %= MOD;

        let mut x2 = 1;
        x2 = n * (n - 1);
        x2 %= MOD;

        let denominator = cm_modular::mod_inverse(x2 as i64, MOD as i64) as usize;
        let ans = (x1 * denominator) % MOD;

        println!("{ans}");
    }
}
