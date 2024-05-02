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
        let (n) = (cin.next::<usize>());

        let mut a = vec![];
        let mut b = vec![];

        for _ in 0..n {
            let x = cin.next::<usize>();
            a.push(x);
        }

        for _ in 0..n {
            let x = cin.next::<usize>();
            b.push(x);
        }

        let mut cur_a = 0;
        let mut cur_b = 0;
        let mut ans = 0;

        while true {
            if a[cur_a] <= b[cur_b] {
                cur_a += 1;
                cur_b += 1;
            } else {
                ans += 1;
                cur_b += 1;
            }

            if cur_a == n {
                ans += n - cur_b;
                break;
            }

            if cur_b == n {
                break;
            }
        }

        println!("{ans}");
    }
}
