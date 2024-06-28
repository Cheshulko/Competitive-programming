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

    let _t = 1;
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![(0, 0); n];
        let mut ba = vec![(0, 0); n];

        for i in 0..n {
            a[i] = (cin.next::<i128>(), i);
        }

        for i in 0..n {
            let b = cin.next::<i128>();
            ba[i] = (a[i].0 - b, a[i].0);
        }

        ba.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => b.1.cmp(&a.1),
            x => x,
        });

        let mut bb = vec![];
        let mut hs = HashSet::<i128>::new();
        for i in 0..n {
            if !hs.contains(&ba[i].1) {
                hs.insert(ba[i].1);
                bb.push(ba[i]);
            }
        }

        let mut h = BinaryHeap::<(i128, i128)>::new();
        for _ in 0..m {
            h.push((cin.next::<i128>(), 1))
        }

        let mut ans = 0;
        let mut cur = 0;

        while let Some((mut p, mut mm)) = h.pop() {
            while let Some(&(pp, mmm)) = h.peek() {
                if pp != p {
                    break;
                } else {
                    mm += mmm;
                    h.pop();
                }
            }
            while cur < bb.len() && bb[cur].1 > p {
                cur += 1;
            }
            if cur == bb.len() {
                break;
            }

            let d = p - bb[cur].1;
            let c = d / bb[cur].0;
            ans += c * 2 * mm;
            p -= c * bb[cur].0;
            assert!(p >= bb[cur].1);
            ans += 2 * mm;
            p -= bb[cur].0;
            assert!(p < bb[cur].1);
            h.push((p, mm));
        }

        println!("{ans}");
    }
}
