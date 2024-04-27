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
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut cnt = vec![0; 101];
        for _ in 0..n {
            let x = cin.next::<usize>();
            cnt[x] += 1;
        }

        let mut up = 0;
        let mut sum = 0;
        loop {
            for x in cnt.iter_mut() {
                up += (k - 1) * (*x / k);
                *x = *x % k;
            }

            sum = 0;
            for x in cnt.iter_mut() {
                if *x == 0 {
                    continue;
                }
                sum += *x;

                let need = k - *x;
                if up >= need {
                    up -= need;
                    up += k - 1;
                    *x = 0;
                } else {
                    *x += up;
                    up = 0;
                }
            }

            if sum == 0 || up == 0 {
                break;
            }
        }

        if up == 0 {
            println!("{sum}");
        } else {
            if up >= k {
                up = k - 1;
            }
            println!("{up}");
        }
    }
}
