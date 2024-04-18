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

        let mut a = vec![];
        for _ in 0..n {
            a.push(cin.next::<i64>());
        }

        let mut b = vec![];
        let mut bs = HashMap::<i64, usize>::new();
        for _ in 0..m {
            let x = cin.next::<i64>();
            *bs.entry(x).or_insert(0) += 1;
            b.push(x);
        }

        let mut ans = 0;
        let mut cur = HashMap::<i64, usize>::new();
        let mut good = 0;
        for i in 0..m {
            let x = a[i];
            let in_a = cur.entry(x).or_insert(0);
            *in_a += 1;

            if bs.get(&x).is_some_and(|in_b| *in_a <= *in_b) {
                good += 1;
            }
        }
        if good >= k {
            ans += 1;
        }

        for i in m..n {
            let x_prev = a[i - m];
            let x_prev_val = *cur.get(&x_prev).unwrap();
            cur.entry(x_prev).and_modify(|x| *x -= 1);

            if bs.get(&x_prev).is_some_and(|in_b| x_prev_val <= *in_b) {
                good -= 1;
            }

            let x = a[i];
            let in_a = cur.entry(x).or_insert(0);
            *in_a += 1;

            if bs.get(&x).is_some_and(|in_b| *in_a <= *in_b) {
                good += 1;
            }
            if good >= k {
                ans += 1;
            }
        }

        println!("{ans}");
    }
}
