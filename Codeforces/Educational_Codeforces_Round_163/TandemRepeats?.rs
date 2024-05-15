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
        let s = cin.next::<String>().into_bytes();
        let n = s.len();

        let l = 1;
        let r = s.len() / 2;

        let mut ans = 0;
        for i in l..=r {
            let mut st = 0;
            let mut en = st + i;
            let mut len = 0;

            let mut ds = 0;
            while en < n && len < i {
                if s[st] == s[en] || s[st] == b'?' || s[en] == b'?' {
                    len += 1;
                    st += 1;
                    en += 1;
                } else {
                    ds = st + 1;
                    st = ds;
                    en = st + i;
                    len = 0;
                }
            }

            if len == i {
                ans = i;
            }
        }

        ans *= 2;

        println!("{ans}");
    }
}
