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
        let n = cin.next::<usize>();

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<i32>();
            arr.push(x);
        }
        arr.push(0);

        let arr = arr.into_iter().collect::<HashSet<_>>();
        let mut arr = arr.into_iter().collect::<Vec<_>>();
        arr.sort();

        let n = arr.len();

        let mut turn_alice = false;
        for i in (0..(n - 1)).rev() {
            if arr[i + 1] - arr[i] == 1 {
                turn_alice = !turn_alice;
            } else {
                if turn_alice {
                    //
                } else {
                    turn_alice = !turn_alice;
                }
            }
        }

        if turn_alice {
            println!("Alice");
        } else {
            println!("Bob");
        }
    }
}
