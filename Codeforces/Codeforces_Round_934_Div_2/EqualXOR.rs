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

        let mut right = (1..=(n as i32)).collect::<HashSet<_>>();
        let mut left = HashSet::<i32>::new();
        let mut mbleft = HashSet::<i32>::new();

        for _ in 0..n {
            let x = cin.next::<i32>();

            right.remove(&x);

            if mbleft.contains(&x) {
                mbleft.remove(&x);
                left.insert(x);
            } else {
                mbleft.insert(x);
            }
        }

        for _ in 0..n {
            let _ = cin.next::<i32>();
        }

        let both = (1..=(n as i32))
            .filter(|x| !left.contains(x) && !right.contains(x))
            .collect::<HashSet<_>>();

        let have = left.len().min(k);
        let rest = 2 * (k - k.min(have));

        let mut lp = left.iter();
        for _ in 0..have {
            print!("{x} {x} ", x = lp.next().unwrap());
        }
        for x in both.iter().take(both.len().min(rest)) {
            print!("{x} ");
        }
        println!();

        let mut rp = right.iter();
        for _ in 0..have {
            print!("{x} {x} ", x = rp.next().unwrap());
        }
        for x in both.iter().take(both.len().min(rest)) {
            print!("{x} ");
        }
        println!();
    }
}
