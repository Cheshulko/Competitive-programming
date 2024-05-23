use std::cmp::*;
use std::collections::*;
use std::io::stdin;
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

        let mut bits = vec![vec![0; 20]; n + 1];
        let mut arr = vec![];

        for i in 0..n {
            let x = cin.next::<usize>();
            arr.push(x);

            for b in 0..20 {
                bits[i + 1][b] = bits[i][b] + ((x & (1 << b)) > 0) as i32;
            }
        }

        let mut l = n;
        let mut r = 0;

        while l - r > 1 {
            let m = (l + r) / 2;

            let mut ok = true;
            for k in 1..(n - m + 1) {
                for b in 0..20 {
                    if (bits[m + k][b] - bits[k][b] > 0) != (bits[m][b] > 0) {
                        ok = false;
                    }
                }
            }

            if ok {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
