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
        let (n, mut k) = (cin.next::<usize>(), cin.next::<u64>());

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<u64>();
            arr.push(x);
        }

        arr.sort_unstable();

        let n = arr.len();

        let mut ans_cnt = arr[0];
        let mut last_i = 0;
        for (i, &x) in arr.iter().enumerate() {
            let need = (i as u64) * (x - ans_cnt);
            if k >= need {
                ans_cnt = x;
                k -= need;
            } else {
                if i == 0 {
                    break;
                }

                let can = k / i as u64;

                k -= can * i as u64;
                ans_cnt += can;
                break;
            }
            last_i += 1;
        }

        let more = k / n as u64;
        k -= more * n as u64;
        ans_cnt += more;

        let mut ans = ans_cnt + (n as u64 - 1) * (ans_cnt - 1);
        for i in last_i..n {
            let mut have = arr[i] - ans_cnt;

            while have < 1 && k > 0 {
                have += 1;
                k -= 1;
            }
            have = have.min(1);
            ans += have as u64;
        }

        ans += k;

        println!("{ans}");
    }
}
