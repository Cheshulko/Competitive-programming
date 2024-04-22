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
        let (mut n, mut k) = (cin.next::<i32>(), cin.next::<i64>());

        let mut p: i64 = 1;
        let mut sum = 0;

        if n == 1 {
            println!("{k}");
        } else {
            loop {
                if sum + p <= k {
                    sum += p;
                    p *= 2;
                } else {
                    break;
                }
            }

            let need = k - sum;

            print!("{sum} {need} ");
            n -= 2;
            while n > 0 {
                print!("0 ");
                n -= 1;
            }
            println!();
        }
    }
}
