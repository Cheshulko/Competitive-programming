use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

fn main() {
    let mut cin = Cin::new();

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a: Vec<usize> = vec![0; n];
        let mut pos = vec![vec![]; 1 << 4];
        for i in 0..n {
            let s = cin.next::<String>().into_bytes();
            for j in 0..2 {
                let f = s[j];

                if f == b'B' {
                    a[i] |= 1 << 0;
                } else if f == b'G' {
                    a[i] |= 1 << 1;
                } else if f == b'R' {
                    a[i] |= 1 << 2;
                } else if f == b'Y' {
                    a[i] |= 1 << 3;
                } else {
                    unreachable!()
                }
            }
            assert!(a[i].count_ones() == 2);
            pos[a[i]].push(i);
        }

        let mut vars = vec![];
        for i in 0..4 {
            for j in (i + 1)..4 {
                vars.push((1 << i) | (1 << j));
            }
        }

        for _ in 0..q {
            let (mut x, mut y) = (cin.next::<usize>(), cin.next::<usize>());

            y -= 1;
            x -= 1;

            if x > y {
                swap(&mut x, &mut y);
            }

            let mut ans = usize::MAX;
            if a[x] & a[y] > 0 {
                println!("{ans}", ans = y - x);
            } else {
                for &var in vars.iter() {
                    if (var & a[x] > 0) && (var & a[y] > 0) {
                        {
                            let p1 = pos[var].partition_point(|z| z < &x);
                            if p1 != pos[var].len() {
                                ans = ans.min(y.abs_diff(pos[var][p1]) + x.abs_diff(pos[var][p1]));
                            }

                            if p1 > 0 {
                                ans = ans.min(
                                    y.abs_diff(pos[var][p1 - 1]) + x.abs_diff(pos[var][p1 - 1]),
                                );
                            }
                        }
                    }
                }

                if ans == usize::MAX {
                    println!("-1");
                } else {
                    println!("{ans}");
                }
            }
        }
    }
}
