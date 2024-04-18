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
        let (n, mut k) = (cin.next::<i64>(), cin.next::<i64>());

        let mut arr = vec![];
        for _ in 0..n {
            arr.push(cin.next::<i64>());
        }

        let mut left = 0;
        let mut right = arr.len() - 1;
        let mut ans = 0;

        while k > 0 && left <= right {
            if arr[left] > arr[right] {
                k -= 2 * arr[right];
                arr[left] -= arr[right];
                right -= 1;
                if k >= 0 {
                    ans += 1;
                }
            } else if arr[left] < arr[right] {
                k -= 2 * arr[left];
                arr[right] -= arr[left];
                left += 1;
                if k >= -1 {
                    ans += 1;
                }
            } else {
                if left == right {
                    k -= arr[left];
                    left += 1;
                    if k >= 0 {
                        ans += 1;
                    }
                } else {
                    if k >= 2 * arr[left] {
                        k -= 2 * arr[left];
                        arr[right] -= arr[left];
                        left += 1;
                        right -= 1;
                        ans += 2;
                    } else if k >= 2 * arr[left] - 1 {
                        k -= 2 * arr[left] - 1;
                        left += 1;
                        ans += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        println!("{ans}");
    }
}
