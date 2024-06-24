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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                a[i][j] = cin.next::<i32>();
            }
        }

        for i in 0..n {
            for j in 0..m {
                let i = i as i32;
                let j = j as i32;
                let nei = [
                    *a.get((i - 1) as usize)
                        .unwrap_or(&vec![])
                        .get(j as usize)
                        .unwrap_or(&i32::MIN),
                    *a.get((i + 1) as usize)
                        .unwrap_or(&vec![])
                        .get(j as usize)
                        .unwrap_or(&i32::MIN),
                    *a.get(i as usize)
                        .unwrap_or(&vec![])
                        .get((j - 1) as usize)
                        .unwrap_or(&i32::MIN),
                    *a.get(i as usize)
                        .unwrap_or(&vec![])
                        .get((j + 1) as usize)
                        .unwrap_or(&i32::MIN),
                ];

                let mn: i32 = nei.into_iter().max().unwrap();
                if mn < a[i as usize][j as usize] {
                    a[i as usize][j as usize] = mn;
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                print!("{x} ", x = a[i][j]);
            }
            println!();
        }
    }
}
