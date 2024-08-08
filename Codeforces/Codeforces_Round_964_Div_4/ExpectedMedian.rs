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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Self {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];

        inv[1] = 1;

        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }

        fact[0] = 1;
        inv_fact[0] = 1;

        for i in 0..max {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }

        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }

        Self {
            fact,
            inv_fact,
            modulo,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        if x < y {
            return 0;
        }

        self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        const MOD: usize = 1_000_000_000 + 7;

        let comb = Combination::new(n, MOD);

        let mut cnt = vec![0; 2];
        for _ in 0..n {
            let x = cin.next::<usize>();
            cnt[x] += 1;
        }

        let mut ans = 0;
        for i in (k / 2) + 1..=k {
            ans += comb.get(cnt[1], i) * comb.get(cnt[0], k - i);
            ans %= MOD;
        }

        println!("{ans}");
    }
}
