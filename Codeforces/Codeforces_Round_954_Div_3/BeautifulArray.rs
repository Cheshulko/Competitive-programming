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
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        a.sort_by(|x, y| match (x % k).cmp(&(y % k)) {
            Ordering::Equal => x.cmp(&y),
            r => r,
        });

        let mut can = true;
        let mut can_odd = n % 2 == 1;
        let mut ans = 0;

        let mut i = 0;
        while i < n {
            let r = a[i] % k;
            let mut cnt = 0;
            while i < n && a[i] % k == r {
                i += 1;
                cnt += 1;
            }

            if cnt % 2 == 0 {
                for j in ((i - cnt)..(i - 1)).step_by(2) {
                    ans += (a[j + 1] - a[j]) / k;
                }
            } else {
                if can_odd {
                    can_odd = false;
                } else {
                    can = false;
                    break;
                }
                let mut pref = vec![0; cnt + 2];
                let mut suf = vec![0; cnt + 2];

                let d = 2;
                for p in (0..(cnt - 1)).step_by(2) {
                    pref[p + d] = pref[p] + (a[i - cnt + p + 1] - a[i - cnt + p]);
                }
                for p in (1..cnt).step_by(2).rev() {
                    suf[p] = suf[p + d] + (a[i - cnt + p + 1] - a[i - cnt + p]);
                }

                let mut lans = usize::MAX;
                for p in (0..cnt).step_by(2) {
                    lans = lans.min(pref[p] + suf[p + 1]);
                }
                ans += lans / k;
            }
        }

        if !can {
            println!("-1");
        } else {
            println!("{ans}");
        }
    }
}
