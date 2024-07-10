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
        let n = cin.next::<usize>();

        let mut a = vec![];
        for i in 0..n {
            let x = cin.next::<u64>();

            if x != 0 {
                a.push(x);
            }
        }

        let n = a.len();

        let mut bits = vec![HashSet::<usize>::new(); 60];

        for i in 0..n {
            for b in 0..60 {
                if a[i] & (1 << b) > 0 {
                    bits[b].insert(i);
                }
            }
        }

        if bits.iter().any(|s| s.len() > 2) {
            println!("3");
            continue;
        }

        let mut ans = usize::MAX;
        for i in 0..n {
            let mut dist = vec![usize::MAX; n];
            let mut par = vec![-1; n];

            dist[i] = 0;

            let mut q = VecDeque::<usize>::new();
            q.push_back(i);

            while let Some(cur) = q.pop_front() {
                for b in 0..60 {
                    if a[cur] & (1 << b) > 0 {
                        for to in 0..n {
                            if to == cur {
                                continue;
                            }

                            if a[to] & (1 << b) > 0 {
                                if dist[to] == usize::MAX {
                                    dist[to] = 1 + dist[cur];
                                    par[to] = cur as i32;
                                    q.push_back(to);
                                } else {
                                    if par[cur] != to as i32 && par[to] != cur as i32 {
                                        ans = ans.min(dist[cur] + dist[to] + 1);
                                    }
                                }
                            }
                        }
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
