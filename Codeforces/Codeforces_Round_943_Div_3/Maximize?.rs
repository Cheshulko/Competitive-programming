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

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
    b
}

fn main() {
    let mut cin = Cin::new();
    let t = cin.next::<usize>();

    for _ in 0..t {
        let n = cin.next::<i32>();

        let mut mx = 1;
        let mut ans = 1;

        for i in 1..n {
            let h = gcd(n, i) + i;
            if h >= mx {
                mx = h;
                ans = i;
            }
        }

        println!("{ans}");
    }
}
