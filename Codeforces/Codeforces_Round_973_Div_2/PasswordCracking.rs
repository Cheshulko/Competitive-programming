use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::usize;
use std::vec;

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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut can = vec![vec![false; n + 4]; 2];

        let mut max = 0;
        let mut max_t = 0;

        let mut in_row = 0;

        {
            let mut cur = String::new();
            for i in 1..=n {
                cur.push('0');
                println!("? {cur}");
                let _ = stdout().flush();
                let x = cin.next::<usize>();

                if x == 1 {
                    in_row += 1;
                    if i > max {
                        max = i;
                        max_t = 0;
                    }
                    can[0][i] = true;
                } else {
                    break;
                }
            }
        }

        {
            can[1][1] = (in_row != n);
            let mut cur = String::from("1");
            for i in 2..=(n - in_row) {
                cur.push('1');
                println!("? {cur}");
                let _ = stdout().flush();
                let x = cin.next::<usize>();

                if x == 1 {
                    if i > max {
                        max = i;
                        max_t = 1;
                    }
                    can[1][i] = true;
                } else {
                    break;
                }
            }
        }

        let mut ans = VecDeque::<char>::new();
        let cur_t = max_t;
        for _ in 0..max {
            if max_t == 0 {
                ans.push_back('0');
            } else {
                ans.push_back('1');
            }
        }

        let mut cur_l = ans.len();

        {
            let mut next_t = cur_t ^ 1;
            let mut next_streak = 0;
            let mut fails = 0;
            let mut pr = false;
            while cur_l < n {
                if can[next_t][next_streak + 1] {
                    if !pr && next_t == cur_t {
                        break;
                    }

                    println!(
                        "? {x}",
                        x = format!(
                            "{xx}{next_t}",
                            xx = ans.clone().into_iter().collect::<String>()
                        )
                    );
                    let _ = stdout().flush();
                    let x = cin.next::<usize>();

                    if x == 1 {
                        fails = 0;
                        cur_l += 1;
                        next_streak += 1;
                        pr = true;
                        ans.push_back(if next_t == 0 { '0' } else { '1' });
                    } else {
                        fails += 1;
                        next_t ^= 1;
                        next_streak = 0;
                    }

                    if fails == 2 {
                        break;
                    }
                } else {
                    fails = 1;
                    next_t ^= 1;
                    next_streak = 0;
                }
            }
        }

        {
            let mut next_t = cur_t ^ 1;
            let mut next_streak = 0;
            let mut fails = 0;
            let mut pr = false;
            while cur_l < n {
                if can[next_t][next_streak + 1] {
                    if !pr && next_t == cur_t {
                        break;
                    }
                    println!(
                        "? {x}",
                        x = format!(
                            "{next_t}{xx}",
                            xx = ans.clone().into_iter().collect::<String>()
                        )
                    );
                    let _ = stdout().flush();
                    let x = cin.next::<usize>();

                    if x == 1 {
                        fails = 0;
                        cur_l += 1;
                        next_streak += 1;
                        pr = true;
                        ans.push_front(if next_t == 0 { '0' } else { '1' });
                    } else {
                        fails += 1;
                        next_t ^= 1;
                        next_streak = 0;
                    }

                    if fails == 2 {
                        break;
                    }
                } else {
                    fails = 1;
                    next_t ^= 1;
                    next_streak = 0;
                }
            }
        }

        assert!(cur_l == n);
        let ans = ans.into_iter().collect::<String>();
        println!("! {ans}");
        let _ = stdout().flush();
    }
}
