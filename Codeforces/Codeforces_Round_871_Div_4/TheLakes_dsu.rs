mod cm_dsu {
    pub struct DSU {
        parents: Vec<usize>,
        ranks: Vec<usize>,
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x != self.parents[x] {
                self.parents[x] = self.find(self.parents[x]);
            }

            self.parents[x]
        }

        pub fn same(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            x == y
        }

        pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            if x == y {
                return false;
            }

            if self.ranks[x] < self.ranks[y] {
                std::mem::swap(&mut y, &mut x);
            }

            self.parents[y] = x;
            self.ranks[x] += self.ranks[y];

            true
        }
    }
}

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

        let mut d = vec![0; n * m + 1];
        let mut mat = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                mat[i][j] = cin.next::<i64>();
            }
        }

        let mut dsu = cm_dsu::DSU::new(n * m + 1);

        if mat[0][0] != 0 {
            d[dsu.find(0)] += mat[0][0];
        }

        for i in 1..n {
            if mat[i][0] == 0 {
                continue;
            }
            if mat[i - 1][0] != 0 {
                dsu.union((i - 1) * m + 0, i * m + 0);
            }
            d[dsu.find(i * m + 0)] += mat[i][0];
        }

        for j in 1..m {
            if mat[0][j] == 0 {
                continue;
            }
            if mat[0][j - 1] != 0 {
                dsu.union(j - 1, j);
            }
            d[dsu.find(j)] += mat[0][j];
        }

        for i in 1..n {
            for j in 1..m {
                if mat[i][j] == 0 {
                    continue;
                }

                if mat[i - 1][j] != 0 {
                    dsu.union((i - 1) * m + j, i * m + j);
                }
                if mat[i][j - 1] != 0 {
                    if !dsu.same(i * m + j - 1, i * m + j) {
                        let s1 = d[dsu.find(i * m + j - 1)];
                        let s2 = d[dsu.find(i * m + j)];

                        dsu.union(i * m + j - 1, i * m + j);
                        d[dsu.find(i * m + j)] = s1 + s2;
                    }
                }
                d[dsu.find(i * m + j)] += mat[i][j];
            }
        }

        let ans = d.into_iter().max().unwrap();
        println!("{ans}");
    }
}
