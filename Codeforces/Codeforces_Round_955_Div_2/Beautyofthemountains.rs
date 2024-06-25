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

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 && b == 0 {
        return 0;
    }
    if a > b {
        swap(&mut a, &mut b);
    }
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
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut a = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                a[i][j] = cin.next::<usize>();
            }
        }

        let mut s = vec![vec![]; n];
        for i in 0..n {
            s[i] = cin.next::<String>().into_bytes();
        }

        let mut s0 = 0;
        let mut s1 = 0;
        for i in 0..n {
            for j in 0..m {
                if s[i][j] == b'0' {
                    s0 += a[i][j];
                } else {
                    s1 += a[i][j];
                }
            }
        }
        let d = s0 as i64 - s1 as i64;

        let mut pref0 = vec![vec![0; m + 1]; n];
        let mut pref1 = vec![vec![0; m + 1]; n];
        for i in 0..n {
            for j in 0..m {
                if s[i][j] == b'0' {
                    pref0[i][j + 1] = pref0[i][j] + 1;
                    pref1[i][j + 1] = pref1[i][j];
                } else {
                    pref0[i][j + 1] = pref0[i][j];
                    pref1[i][j + 1] = pref1[i][j] + 1;
                }
            }
        }
        let mut g = 0;
        for i in 0..=(n - k) {
            for j in 0..=(m - k) {
                let mut sum = 0;
                for p in 0..k {
                    sum += (pref0[i + p][j + k] - pref0[i + p][j])
                        - (pref1[i + p][j + k] - pref1[i + p][j]);
                }

                g = gcd(g, (sum as i64).abs());
            }
        }

        if d == 0 || (g != 0 && d % g == 0) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
