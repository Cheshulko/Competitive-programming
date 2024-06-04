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

        let mut a = vec![];
        let mut b = vec![];

        for _ in 0..(n + m + 1) {
            let x = cin.next::<usize>();
            a.push(x);
        }

        for _ in 0..(n + m + 1) {
            let x = cin.next::<usize>();
            b.push(x);
        }

        let mut un = BTreeSet::<usize>::new();
        let mut um = BTreeSet::<usize>::new();
        let mut s = 0;

        let mut nn = BTreeSet::<usize>::new();
        let mut mm = BTreeSet::<usize>::new();

        for i in 0..(n + m) {
            if a[i] > b[i] {
                nn.insert(i);

                if un.len() < n {
                    s += a[i];
                    un.insert(i);
                } else {
                    assert!(um.len() < m);
                    s += b[i];
                    um.insert(i);
                }
            } else {
                mm.insert(i);

                if um.len() < m {
                    s += b[i];
                    um.insert(i);
                } else {
                    assert!(un.len() < n);
                    s += a[i];
                    un.insert(i);
                }
            }
        }

        for i in 0..(n + m) {
            let mut ans = s;
            if un.contains(&i) {
                ans -= a[i];
                if let Some(&to_add) = nn.range((*un.last().unwrap() + 1)..).next() {
                    ans += a[to_add];
                    ans -= b[to_add];
                    ans += b[n + m];
                } else {
                    ans += a[n + m];
                }
            } else {
                ans -= b[i];
                if let Some(&to_add) = mm.range((*um.last().unwrap() + 1)..).next() {
                    ans += b[to_add];
                    ans -= a[to_add];
                    ans += a[n + m];
                } else {
                    ans += b[n + m];
                }
            }
            print!("{ans} ");
        }

        println!("{s}");
    }
}
