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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Type {
    None,
    Value,
    Range,
    Ranges(usize),
}

fn set_range(i: usize, j: usize, arr: &mut [i64], ops: &mut Vec<(usize, usize)>) {
    assert!(j >= i);

    let mut cur_i = i;
    let mut cur_j = i;

    loop {
        if cur_j == j + 1 {
            break;
        }

        if arr[cur_j] != 0 {
            arr[cur_j] = 0;
            ops.push((cur_j, cur_j));
        }

        let mut up = 1;
        while cur_i >= i + 1 && arr[cur_i - 1] == arr[cur_i] + 1 {
            up += 1;
            cur_i -= 1;
        }

        if cur_i >= i + 1 {
            cur_i -= 1;
        }

        ops.push((cur_i, cur_j));

        for k in cur_i..=cur_j {
            arr[k] = up;
        }

        if cur_i == i {
            cur_j += 1;
        }

        cur_i = cur_j;
    }
}

fn main() {
    let mut cin = Cin::new();

    let n = cin.next::<usize>();

    let mut a = vec![];
    for _ in 0..n {
        a.push(cin.next::<i64>());
    }

    let mut dp: Vec<(i64, Type)> = vec![(0, Type::None); n + 1];
    for i in 1..=n {
        let x = dp[i - 1].0 + a[i - 1];
        if dp[i].0 < x {
            dp[i] = (x, Type::Value);
        }

        let x = (i * i) as i64;
        if dp[i].0 < x {
            dp[i] = (x, Type::Range);
        }
        for j in 0..i {
            let x = dp[j].0 + ((i - j) * (i - j)) as i64;
            if dp[i].0 < x {
                dp[i] = (x, Type::Ranges(j));
            }
        }
    }

    let mut cur = n;
    let mut ans = vec![];

    while dp[cur].1 != Type::None {
        cur = match dp[cur].1 {
            Type::None => unreachable!(),
            Type::Value => cur - 1,
            Type::Range => {
                set_range(0, cur - 1, &mut a, &mut ans);
                0
            }
            Type::Ranges(j) => {
                set_range(j, cur - 1, &mut a, &mut ans);
                j
            }
        }
    }

    println!("{} {}", dp[n].0, ans.len());
    for x in ans.into_iter() {
        println!("{} {}", x.0 + 1, x.1 + 1);
    }
}
