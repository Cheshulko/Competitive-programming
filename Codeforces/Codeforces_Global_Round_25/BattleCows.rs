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
        let (n, mut k) = (cin.next::<usize>(), cin.next::<usize>());
        k -= 1;

        let mut arr = vec![0; n];
        for i in 0..n {
            let x = cin.next::<usize>();
            arr[i] = x;
        }

        let mut ans = 0;
        for i in 0..n {
            if arr[i] <= arr[k] {
                ans += 1;
            } else {
                break;
            }
        }
        if ans > 0 {
            ans -= 1;
        }

        for i in 0..n {
            if arr[i] > arr[k] {
                if i < k {
                    let t = arr[i];
                    arr[i] = arr[k];
                    arr[k] = t;
                    k = i;
                }
                break;
            }
        }

        let mut ans2 = 0;
        for i in (k + 1)..n {
            if arr[i] <= arr[k] {
                ans2 += 1;
            } else {
                break;
            }
        }

        if k != 0 {
            ans2 += 1;
        }

        ans = ans.max(ans2);

        println!("{ans}");
    }
}
