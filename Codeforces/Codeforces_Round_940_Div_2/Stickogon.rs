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

        let mut arr = vec![0; 101];

        for i in 0..n {
            let x = cin.next::<usize>();
            arr[x] += 1;
        }

        let mut ans = 0;

        for i in 0..101 {
            ans += arr[i] / 3;
        }

        println!("{ans}");
    }
}
