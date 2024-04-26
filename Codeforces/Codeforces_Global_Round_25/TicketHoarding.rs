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
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<usize>();
            arr.push(x);
        }

        let mut arr = arr
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<_>>();

        arr.sort();

        let need_days = k / m + (k % m != 0) as usize;

        let mut arr = arr
            .into_iter()
            .take(need_days)
            .map(|(x, i)| (i, x))
            .collect::<Vec<_>>();

        let small_day_cnt = if k % m != 0 { k % m } else { m };
        let small_day_ind = arr[need_days - 1].0;

        arr.sort();

        let mut have = 0;
        let mut ans = 0;
        for i in 0..need_days {
            if arr[i].0 == small_day_ind {
                ans += (arr[i].1 + have) * small_day_cnt;
                have += small_day_cnt;
            } else {
                ans += (arr[i].1 + have) * m;
                have += m;
            }
        }

        println!("{ans}");
    }
}
