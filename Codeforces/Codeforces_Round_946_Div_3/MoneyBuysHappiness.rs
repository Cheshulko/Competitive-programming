use std::cmp::*;
use std::collections::*;
use std::io::stdin;
use std::mem::swap;
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

    let mut dp_prev = BTreeMap::<i64, i64>::new();

    for _ in 0..t {
        let (m, x) = (cin.next::<usize>(), cin.next::<i64>());

        let mut c = vec![];
        let mut h = vec![];

        for _ in 0..m {
            let cc = cin.next::<i64>();
            let hh = cin.next::<i64>();
            c.push(cc);
            h.push(hh);
        }

        dp_prev.clear();
        dp_prev.insert(0, 0);

        for day in 1..=m {
            let mut dp_cur = dp_prev.clone();

            for (&happiness, &money) in dp_prev.iter() {
                if money < c[day - 1] {
                    continue;
                }
                let dx = money - c[day - 1];
                let hh = happiness + h[day - 1];
                let prev_money = dp_cur.entry(hh).or_insert(0);
                *prev_money = (*prev_money).max(dx);
            }

            for (_, money) in dp_cur.iter_mut() {
                *money += x;
            }
            swap(&mut dp_prev, &mut dp_cur);
        }

        let ans = dp_prev.keys().max().unwrap();
        println!("{ans}");
    }
}
