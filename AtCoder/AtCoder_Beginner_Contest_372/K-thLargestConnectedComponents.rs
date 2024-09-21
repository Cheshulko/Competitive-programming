use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;
use std::vec;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &Path) -> Self {
        let tokens = VecDeque::new();
        let file = File::open(&path).expect("Expect file exists");
        Self {
            reader: Box::new(BufReader::new(file)),
            tokens,
        }
    }
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self {
            reader: Box::new(BufReader::new(std::io::stdin())),
            tokens,
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            self.reader.read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

mod cm {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = vec![BTreeSet::<usize>::new(); n];
        for i in 0..n {
            s[i].insert(i);
        }
        let mut dsu = cm::DSU::new(n);

        for _ in 0..q {
            let t = cin.next::<usize>();
            if t == 1 {
                let (v, u) = (cin.next::<usize>(), cin.next::<usize>());

                let dv = dsu.find(v - 1);
                let du = dsu.find(u - 1);
                if dv != du {
                    dsu.union(v - 1, u - 1);
                    let nd = dsu.find(v - 1);
                    if nd == dv {
                        let gg = s[du].iter().map(|x| *x).collect::<Vec<_>>();
                        for x in gg.into_iter() {
                            s[dv].insert(x);
                        }
                    } else {
                        assert!(nd == du);
                        let gg = s[dv].iter().map(|x| *x).collect::<Vec<_>>();
                        for x in gg.into_iter() {
                            s[du].insert(x);
                        }
                    }
                }
            } else {
                let (v, k) = (cin.next::<usize>(), cin.next::<usize>());
                let dv = dsu.find(v - 1);

                if s[dv].len() < k {
                    println!("-1");
                } else {
                    let it = s[dv].iter().rev().skip(k - 1).next().unwrap() + 1;
                    println!("{it}");
                }
            }
        }
    }

    Ok(())
}
