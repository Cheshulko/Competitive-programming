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

    const MAX: usize = 2 * 100000 + 1;

    fn dif(s1: &[u8], s2: &[u8]) -> usize {
        assert!(s1.len() == s2.len());

        s1.iter().zip(s2.iter()).filter(|(a, b)| (a != b)).count()
    }

    for _ in 0..t {
        let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();

        let mut ans = n;
        'out: for i in 1..n {
            if n % i == 0 {
                let cnt = n / i;

                let x = [&s[..i], &s[i..(2 * i)]];
                for x_ in x.into_iter() {
                    let mut difs = 0;
                    for j in 0..cnt {
                        difs += dif(x_, &s[(j * i)..((j + 1) * i)]);

                        if difs > 1 {
                            break;
                        }
                    }

                    if difs <= 1 {
                        ans = i;
                        break 'out;
                    }
                }
            }
        }

        println!("{ans}");
    }
}
