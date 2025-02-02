use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;

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

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let max = 200_000 + 5;
        let mut f = cm_fenwick::Fenwick::new(max);
        let mut cnt_f = cm_fenwick::Fenwick::new(max);

        cnt_f.add(0, 1);

        let mut s = 0;
        let mut ans = 0;
        let mut pref = 0;
        for i in 0..n {
            s += a[i];
            s %= m;

            let less = f.sum(0..s);
            let gr = f.sum((s + 1)..max);

            let cnt_less = cnt_f.sum(0..s);
            let cnt_gr = cnt_f.sum((s + 1)..max);

            ans += s * cnt_less - less;
            ans += (s + m) * cnt_gr - gr;

            pref += a[i];
            pref %= m;

            f.add(pref, pref);
            cnt_f.add(pref, 1);
        }

        println!("{ans}");
    }

    Ok(())
}
