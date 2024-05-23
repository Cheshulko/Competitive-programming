use std::cmp::*;
use std::collections::*;
use std::io::stdin;
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
    let t = cin.next::<usize>();

    for _ in 0..t {
        let (a, b, n, m) = (
            cin.next::<i64>(),
            cin.next::<i64>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut xd = BTreeMap::<i64, Vec<usize>>::new();
        let mut yd = BTreeMap::<i64, Vec<usize>>::new();
        let mut used = vec![false; n];

        for i in 0..n {
            let (y, x) = (cin.next::<i64>(), cin.next::<i64>());

            xd.entry(x).or_insert(vec![]).push(i);
            yd.entry(y).or_insert(vec![]).push(i);
        }

        let mut cut_y_u = 0;
        let mut cut_y_d = a + 1;
        let mut cut_x_l = 0;
        let mut cut_x_r = b + 1;

        let mut ans = vec![0; 2];
        let mut turn = 0;

        for i in 0..m {
            let (c, k) = (cin.next::<char>(), cin.next::<i64>());

            let la = match c {
                'U' => {
                    let to_cut = cut_y_u + k;
                    let res = yd.range((cut_y_u + 1)..=to_cut).rev().collect::<Vec<_>>();
                    cut_y_u = to_cut;

                    res
                }
                'D' => {
                    let to_cut = cut_y_d - k;
                    let res = yd.range(to_cut..cut_y_d).collect::<Vec<_>>();
                    cut_y_d = to_cut;

                    res
                }
                'L' => {
                    let to_cut = cut_x_l + k;
                    let res = xd.range((cut_x_l + 1)..=to_cut).rev().collect::<Vec<_>>();
                    cut_x_l = to_cut;

                    res
                }
                'R' => {
                    let to_cut = cut_x_r - k;
                    let res = xd.range(to_cut..cut_x_r).collect::<Vec<_>>();
                    cut_x_r = to_cut;

                    res
                }
                _ => unreachable!(),
            };

            for (_, inds) in la.into_iter() {
                for &ind in inds {
                    if !used[ind] {
                        used[ind] = true;
                        ans[turn] += 1;
                    }
                }
            }

            turn ^= 1;
        }

        println!("{a} {b}", a = ans[0], b = ans[1]);
    }
}
