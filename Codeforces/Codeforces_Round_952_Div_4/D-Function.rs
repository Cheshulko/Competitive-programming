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

const MOD: i64 = 1_000_000_000 + 7;

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

fn calc(x: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n == 1 {
        (1 + x) % MOD
    } else {
        let e = cm_modular::modular_exponential(x, n + 1, MOD);
        let x_ = (MOD + e - x) % MOD;
        let y_ = cm_modular::mod_inverse(x - 1, MOD);
        (1 + (x_ * y_)) % MOD
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let (l, r, k) = (cin.next::<i64>(), cin.next::<i64>(), cin.next::<i64>());

        if k > 9 {
            println!("0");
            continue;
        }

        let x = 9 / k;

        let mut ans = calc(x + 1, r - 1);
        if l >= 1 {
            ans = (MOD + ans - calc(x + 1, l - 1)) % MOD;
        }
        ans = (ans * x) % MOD;

        println!("{ans}");
    }
}
