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

    const MAX: usize = 5 * 100000 + 1;
    let mut dp = [false; MAX];

    for mut x in 0..MAX {
        let x_ = x;
        let mut can = true;
        while x > 0 {
            if x % 10 > 1 {
                can = false;
                break;
            }
            x /= 10;
        }
        dp[x_] = can;
    }

    fn go(dp: &mut [bool], cur: usize) -> bool {
        if dp[cur] {
            return true;
        }

        let mut can = false;
        for x in 2..=((cur as f64).sqrt() as usize) {
            if cur % x == 0 && dp[x] {
                can |= go(dp, cur / x);
            }
        }

        dp[cur] = can;
        can
    }

    for _ in 0..t {
        let n = cin.next::<usize>();

        let ans = if go(&mut dp, n) { "YES" } else { "NO" };

        println!("{ans}");
    }
}
