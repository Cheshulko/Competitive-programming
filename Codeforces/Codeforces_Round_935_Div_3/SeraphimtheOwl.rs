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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![];
        let mut b = vec![];

        for _ in 0..n {
            let x = cin.next::<u64>();
            a.push(x);
        }

        for _ in 0..n {
            let x = cin.next::<u64>();
            b.push(x);
        }

        a.reverse();
        b.reverse();

        let mut dp = vec![[u64::MAX; 2]; n + 1];
        dp[0][0] = 0;
        dp[0][1] = 0;
        for i in 1..=n {
            dp[i][0] = (dp[i - 1][0] + b[i - 1]).min(dp[i - 1][1] + b[i - 1]);
            dp[i][1] = (dp[i - 1][0] + a[i - 1]).min(dp[i - 1][1] + a[i - 1]);
        }

        let mut ans = u64::MAX;
        for i in (n - m + 1)..=n {
            ans = ans.min(dp[i][1]);
        }

        println!("{ans}");
    }
}
