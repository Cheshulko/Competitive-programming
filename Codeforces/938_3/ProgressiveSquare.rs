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
        let (n, c, d) = (cin.next::<i64>(), cin.next::<i64>(), cin.next::<i64>());

        let mut hm = HashMap::<i64, usize>::new();
        let mut min = i64::MAX;
        for _ in 0..n * n {
            let x = cin.next::<i64>();
            min = min.min(x);
            *hm.entry(x).or_insert(0) += 1;
        }

        let mut cur = min - c;
        let mut not_ok = false;
        'out: for _ in 0..n {
            cur = cur + c;

            let cur_cnt = hm.entry(cur).or_insert(0);
            if cur_cnt == &0 {
                println!("NO");
                not_ok = true;
                break 'out;
            } else {
                *cur_cnt -= 1;
            }

            let mut next = cur;

            for _ in 0..n - 1 {
                next = next + d;
                let next_cnt = hm.entry(next).or_insert(0);
                if next_cnt == &0 {
                    println!("NO");
                    not_ok = true;
                    break 'out;
                } else {
                    *next_cnt -= 1;
                }
            }
        }

        if !not_ok {
            println!("YES");
        }
    }
}
