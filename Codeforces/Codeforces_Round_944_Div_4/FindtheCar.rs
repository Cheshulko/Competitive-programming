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
        let (_, k, q) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut a = vec![0];
        let mut b = vec![0];

        for _ in 0..k {
            let x = cin.next::<usize>();
            a.push(x);
        }

        for _ in 0..k {
            let x = cin.next::<usize>();
            b.push(x);
        }

        for _ in 0..q {
            let dd = cin.next::<usize>();

            let mut pp = match a.binary_search(&dd) {
                Ok(p) => p,
                Err(p) => p,
            };

            if a[pp] > dd {
                if pp > 0 {
                    pp -= 1;
                }
            }

            let more = dd - a[pp];

            let ans = if more > 0 {
                let next_p = a[pp + 1];
                let next = b[pp + 1];
                let cur_p = a[pp];
                let cur = b[pp];

                let dp = (next_p - cur_p);
                let dt = (next - cur);
                let more = more;

                let tt = (more * dt / dp);

                b[pp] + tt
            } else {
                b[pp]
            };

            println!("{ans}");
        }
    }
}
