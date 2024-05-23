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
        let n = cin.next::<usize>();

        let mut p = vec![];
        for _ in 0..n {
            let x = cin.next::<usize>();
            p.push(x);
        }

        let v1 = {
            let st = 0;
            let ne = st + 1;

            let mut l = vec![];
            let mut h = vec![];

            for i in (st..n).step_by(2) {
                l.push((p[i], i));
            }

            for i in (ne..n).step_by(2) {
                h.push((p[i], i));
            }

            l.sort_unstable();
            h.sort_unstable();

            let mut hi = n;
            let mut lo = 1;

            let mut ans = vec![0; n];
            for (_, i) in l.into_iter().rev() {
                ans[i] = lo;
                lo += 1;
            }
            for (_, i) in h.into_iter() {
                ans[i] = hi;
                hi -= 1;
            }

            ans
        };

        let v2 = {
            let st = 0;
            let ne = st + 1;

            p.reverse();

            let mut l = vec![];
            let mut h = vec![];

            for i in (st..n).step_by(2) {
                l.push((p[i], i));
            }

            for i in (ne..n).step_by(2) {
                h.push((p[i], i));
            }

            l.sort_unstable();
            h.sort_unstable();

            let mut hi = n;
            let mut lo = 1;

            let mut ans = vec![0; n];
            for (_, i) in l.into_iter().rev() {
                ans[i] = lo;
                lo += 1;
            }
            for (_, i) in h.into_iter() {
                ans[i] = hi;
                hi -= 1;
            }

            p.reverse();
            ans.reverse();

            ans
        };

        let mut c1 = 0;
        for (xx, yy) in v1.windows(3).zip(p.windows(3)) {
            c1 += (xx[0] + yy[0] < xx[1] + yy[1] && xx[1] + yy[1] > xx[2] + yy[2]) as i32;
        }

        let mut c2 = 0;
        for (xx, yy) in v2.windows(3).zip(p.windows(3)) {
            c2 += (xx[0] + yy[0] < xx[1] + yy[1] && xx[1] + yy[1] > xx[2] + yy[2]) as i32;
        }

        let ans = if c1 > c2 { v1 } else { v2 };

        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
