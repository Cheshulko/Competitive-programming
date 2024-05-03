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
        let n = cin.next::<usize>();

        let mut a = vec![];
        for _ in 0..(n - 1) {
            let x = cin.next::<usize>();
            a.push(x);
        }

        let mut ans = vec![501];

        for i in 0..(n - 1) {
            ans.push(ans[i] + a[i]);
        }

        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
