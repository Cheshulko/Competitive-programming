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
    let t = cin.next::<i64>();

    for _ in 0..t {
        let n = cin.next::<usize>();

        let mut arr = vec![];
        for _ in 0..n {
            arr.push(cin.next::<i64>());
        }

        if n == 1 || n == 2 {
            println!("-1");
            continue;
        }

        if arr.iter().max() == arr.iter().min() {
            println!("-1");
            continue;
        }

        let mut cnt1 = 0;
        let mut cnt2 = 0;

        let b = arr[0];
        for i in 0..n {
            if arr[i] == b {
                cnt1 += 1;
            } else {
                break;
            }
        }

        let b = arr[n - 1];
        for i in (0..n).rev() {
            if arr[i] == b {
                cnt2 += 1;
            } else {
                break;
            }
        }
        let mut ans = cnt2.min(cnt1);

        let x = arr
            .into_iter()
            .enumerate()
            .filter(|x| x.1 != b)
            .collect::<Vec<_>>();
        let l = x.len();

        for i in 0..l - 1 {
            ans = ans.min(x[i + 1].0 - x[i].0 - 1);
        }

        println!("{ans}");
    }
}
