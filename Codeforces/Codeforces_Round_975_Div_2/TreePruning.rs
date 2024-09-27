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

fn dfs(
    cur: usize,
    d: usize,
    own_deps: &mut Vec<usize>,
    child_deps: &mut Vec<usize>,
    par: i32,
    adj: &Vec<Vec<usize>>,
) -> usize {
    own_deps[d] += 1;

    let mut dc = d;
    for &to in adj[cur].iter() {
        if to as i32 == par {
            continue;
        }

        dc = dc.max(dfs(to, d + 1, own_deps, child_deps, cur as i32, adj));
    }
    child_deps[dc] += 1;

    return dc;
}

mod cm_fenwick {
    // [l; r)
    pub struct Fenwick<T> {
        ary: Vec<T>,
    }

    impl<T: Clone + Default + std::ops::AddAssign<T>> Fenwick<T> {
        /// - Time: O(n)
        /// - Space: O(n)
        pub fn new(n: usize) -> Self {
            Fenwick {
                ary: vec![T::default(); n],
            }
        }

        /// - Time: O(n)
        /// - Space: O(n)
        pub fn build_on_array(a: &[T]) -> Self {
            let mut ary = a.to_vec();
            for i in 0..a.len() {
                let j = i | (i + 1);
                if j < a.len() {
                    let tmp = ary[i].clone();
                    ary[j] += tmp;
                }
            }
            Fenwick { ary }
        }

        fn accum(&self, mut idx: usize) -> T {
            let mut sum = T::default();
            while idx > 0 {
                sum += self.ary[idx - 1].clone();
                idx &= idx - 1;
            }
            sum
        }

        // O(log n)
        pub fn add(&mut self, mut idx: usize, val: T) {
            while idx < self.ary.len() {
                self.ary[idx] += val.clone();
                idx |= idx + 1;
            }
        }

        /// [range.start, range.end). O(log n)
        pub fn sum(&self, range: std::ops::Range<usize>) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            self.accum(range.end) - self.accum(range.start)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut adj = vec![vec![]; n];
        for i in 0..n - 1 {
            let (u, v) = (cin.next::<usize>(), cin.next::<usize>());
            let v = v - 1;
            let u = u - 1;
            adj[v].push(u);
            adj[u].push(v);
        }

        let mut own_deps = vec![0; n + 1];
        let mut child_deps = vec![0; n + 1];
        dfs(0, 1, &mut own_deps, &mut child_deps, -1, &adj);

        let own = cm_fenwick::Fenwick::build_on_array(&own_deps);
        let child = cm_fenwick::Fenwick::build_on_array(&child_deps);

        let mut ans = n;
        for i in 1..=n {
            let x1 = child.sum(1..i);
            let x2 = own.sum((i + 1)..(n + 1));
            ans = ans.min(x1 + x2);
        }
        println!("{ans}");
    }

    Ok(())
}
