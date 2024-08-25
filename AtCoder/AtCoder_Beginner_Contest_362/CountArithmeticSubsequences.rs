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

    const MOD: usize = 998244353;

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<i64>();
        }

        let mut hm = vec![vec![HashMap::<i64, usize>::new(); n + 1]; n + 1];
        for i in 0..n {
            hm[i][1].insert(a[i], 1);
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let dx = a[j] - a[i];

                let x = hm[j][2].entry(dx).or_default();
                *x += 1;
                *x %= MOD;

                for d in 2..=n {
                    let to_add = hm[i][d].clone().into_iter().collect::<Vec<_>>();
                    for (v, cnt) in to_add.into_iter() {
                        if v != dx {
                            continue;
                        }

                        let x = hm[j][d + 1].entry(v).or_default();
                        *x += cnt;
                        *x %= MOD;
                    }
                }
            }
        }

        let mut ans = vec![0; n];

        let mut sums = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            for d in 1..=n {
                for (_, &cnt) in hm[i][d].iter() {
                    sums[i][d] += cnt;
                    sums[i][d] %= MOD;
                }
            }
        }

        for i in 0..n {
            for d in 1..=n {
                ans[d - 1] += sums[i][d];
                ans[d - 1] %= MOD;
            }
        }

        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
