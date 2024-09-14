use std::cmp;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::i64;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
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

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut x = vec![0; n];
        let mut p = vec![0; n];

        let mut hs = BTreeSet::<i64>::new();

        for i in 0..n {
            let mut xx = cin.next::<i64>();

            xx -= 1;

            x[i] = xx;

            hs.insert(xx);
            hs.insert(xx - 1);
            hs.insert(xx + 1);
        }

        for i in 0..n {
            let mut pp = cin.next::<i64>();
            p[i] = pp;
        }

        let q = cin.next::<usize>();
        let mut lr = vec![];
        for i in 0..q {
            let (l, r) = (cin.next::<i64>(), cin.next::<i64>());
            let l = l - 1;
            let r = r - 1;
            lr.push((l, r));

            hs.insert(l);
            hs.insert(l - 1);
            hs.insert(l + 1);

            hs.insert(r);
            hs.insert(r - 1);
            hs.insert(r + 1);
        }

        let mut mp = HashMap::<i64, usize>::new();
        for x in hs.into_iter() {
            if !mp.contains_key(&x) {
                let len = mp.len();
                mp.insert(x, len);
            }
        }

        let mut fenwick = cm_fenwick::Fenwick::new(mp.len() + 1);
        for i in 0..n {
            let x = *mp.get(&x[i]).unwrap();
            fenwick.add(x, p[i]);
        }

        for i in 0..q {
            let l = *mp.get(&lr[i].0).unwrap();
            let r = *mp.get(&lr[i].1).unwrap();

            let mut ans = fenwick.sum(l..(r + 1));
            println!("{ans}");
        }
    }
}
