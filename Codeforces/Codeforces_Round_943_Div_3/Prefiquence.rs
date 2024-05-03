use std::cmp::*;
use std::collections::*;
use std::io::stdin;

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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let a = cin.next::<String>();
        let b = cin.next::<String>();

        let a = a.into_bytes();
        let b = b.into_bytes();

        let bb = b.iter().enumerate().fold(vec![vec![]; 2], |mut v, (p, x)| {
            if x == &b'0' {
                v[0].push(p as i32);
            } else {
                v[1].push(p as i32);
                (p as i32);
            }
            v
        });

        let mut ans = 0;
        let mut cur = -1;
        for i in 0..a.len() {
            let c = a[i];
            let p = if c == b'0' {
                let x = match bb[0].binary_search(&cur) {
                    Ok(p) => p,
                    Err(p) => p,
                };

                if x == bb[0].len() {
                    None
                } else {
                    Some(bb[0][x])
                }
            } else {
                let x = match bb[1].binary_search(&cur) {
                    Ok(p) => p,
                    Err(p) => p,
                };

                if x == bb[1].len() {
                    None
                } else {
                    Some(bb[1][x])
                }
            };

            if let Some(p) = p {
                ans += 1;
                cur = p as i32 + 1;
            } else {
                break;
            }
        }

        println!("{ans}");
    }
}
