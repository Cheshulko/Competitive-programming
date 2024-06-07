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
    // let _t = cin.next::<usize>();
    let _t = 1;

    for _ in 0.._t {
        let (n, m, d) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut c = VecDeque::new();
        for _ in 0..m {
            let x = cin.next::<usize>();
            c.push_back(x);
        }

        let mut p = vec![0; n + 2];
        let mut cv = VecDeque::new();
        let mut end = n;
        for i in (0..m).rev() {
            let cc = c[i];
            let to = end - cc + 1;
            cv.push_front(to);
            p[to] = cc;
            end = to - 1;
        }

        let mut pos = 0;
        let mut can = true;
        let mut cvv = vec![];
        let mut cc = vec![];
        loop {
            let next = pos + d;
            if let Some(x) = cv.pop_front() {
                if next < x {
                    cvv.push(next);
                    let l = c.pop_front().unwrap();
                    cc.push(l);
                    pos = next + (l - 1);
                } else {
                    cvv.push(x);
                    let l = c.pop_front().unwrap();
                    cc.push(l);
                    pos = x + (l - 1);
                }
            } else {
                if pos + d < n + 1 {
                    can = false;
                }
                break;
            }
        }

        if can {
            println!("YES");
            let mut ans = vec![0; n];
            for (i, x) in cvv.into_iter().enumerate() {
                let l = cc[i];
                for j in 0..l {
                    ans[x + j - 1] = i + 1;
                }
            }
            for x in ans.into_iter() {
                print!("{x} ");
            }
            println!();
        } else {
            println!("NO");
        }
    }
}
