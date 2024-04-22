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

    let M: i64 = 1_000_000_000 + 7;

    for _ in 0..t {
        let (mut n, k) = (cin.next::<usize>(), cin.next::<i64>());

        let mut cnt = vec![0; (n + 1).max(3)];

        for _ in 0..k {
            let r = cin.next::<i32>();
            let c = cin.next::<i32>();

            if r == c {
                n -= 1;
            } else {
                n -= 2;
            }
        }

        cnt[0] = 1;
        cnt[1] = 1;
        cnt[2] = 2 + 1;

        for i in 3..=n {
            cnt[i] = (cnt[i - 1] + 2 * (i - 1) as i64 * cnt[i - 2]) % M;
        }

        println!("{}", cnt[n]);
    }
}
