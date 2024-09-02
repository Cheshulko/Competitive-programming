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

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();

        let mut pref = vec![vec![0; 30]; n];
        let mut suf = vec![vec![0; 30]; n];

        for i in 0..n {
            if i >= 2 {
                for c in 0..30 {
                    pref[i][c] = pref[i - 2][c];
                }
                pref[i][(s[i] - b'a') as usize] += 1;
            } else {
                pref[i][(s[i] - b'a') as usize] += 1;
            }
        }

        for i in (0..n).rev() {
            if i + 2 < n {
                for c in 0..30 {
                    suf[i][c] = suf[i + 2][c];
                }
                suf[i][(s[i] - b'a') as usize] += 1;
            } else {
                suf[i][(s[i] - b'a') as usize] += 1;
            }
        }

        let mut ans = usize::MAX;
        let mut ans2 = 0;

        {
            let mut mx = 0;
            let mut end = n - 1;
            if end % 2 == 1 {
                end -= 1;
            }
            let mut ss = 0;
            for c in 0..30 {
                mx = mx.max(pref[end][c]);
                ss += pref[end][c];
            }

            ans2 += ss - mx;
        }
        'out: {
            let mut mx = 0;
            let mut end = n - 1;
            if end % 2 == 0 {
                if end > 0 {
                    end -= 1;
                } else {
                    break 'out;
                }
            }
            let mut ss = 0;
            for c in 0..30 {
                mx = mx.max(pref[end][c]);
                ss += pref[end][c];
            }

            ans2 += ss - mx;
        }

        if n % 2 == 0 {
            ans = ans2;
        }

        if n % 2 == 1 {
            for i in 0..n {
                let mut s0 = 0;
                let mut s1 = 0;
                if i > 0 {
                    for c in 0..30 {
                        s0 += pref[i - 1][c];
                    }
                }
                if i + 2 < n {
                    for c in 0..30 {
                        s0 += suf[i + 2][c];
                    }
                }
                if i > 1 {
                    for c in 0..30 {
                        s1 += pref[i - 2][c];
                    }
                }
                if i + 1 < n {
                    for c in 0..30 {
                        s1 += suf[i + 1][c];
                    }
                }

                for c1 in 0..30 {
                    let mut x0 = s0;

                    if i > 0 {
                        x0 -= pref[i - 1][c1];
                    }
                    if i + 2 < n {
                        x0 -= suf[i + 2][c1];
                    }

                    for c2 in 0..30 {
                        let mut x1 = s1;

                        if i > 1 {
                            x1 -= pref[i - 2][c2];
                        }
                        if i + 1 < n {
                            x1 -= suf[i + 1][c2];
                        }

                        ans = ans.min(1 + x0 + x1);
                    }
                }
            }
        }

        println!("{ans}");
    }
}
