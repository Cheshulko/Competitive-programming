use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
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

#[derive(Clone, Copy, Debug)]
struct Step {
    com_min: usize,
    com_max: usize,
    ind_ans: usize,
    prev_ind_ans: usize,
}

impl Default for Step {
    fn default() -> Self {
        Self {
            com_min: usize::MAX,
            com_max: usize::MIN,
            ind_ans: usize::MAX,
            prev_ind_ans: 0,
        }
    }
}

fn main() {
    let mut cin = Cin::new();
    // let _t = cin.next::<usize>();
    let _t = 1;

    for _ in 0.._t {
        let n = cin.next::<usize>();
        let mut a = vec![];

        for i in 0..n {
            let x = cin.next::<usize>();
            a.push((x, i));
        }
        a.sort_unstable();

        let mut dp = vec![Step::default(); n + 1];
        for i in 1..=3 {
            dp[i].com_min = dp[i - 1].com_min.min(a[i - 1].0);
            dp[i].com_max = dp[i - 1].com_max.max(a[i - 1].0);
            dp[i].ind_ans = dp[i].com_max - dp[i].com_min;
        }

        for i in 4..=n {
            let can_create_next = n - i >= 2;

            let to_cur_min = dp[i - 1].com_min.min(a[i - 1].0);
            let to_cur_max = dp[i - 1].com_max.max(a[i - 1].0);
            let to_cur_ans = to_cur_max - to_cur_min;

            if dp[i].ind_ans + dp[i].prev_ind_ans >= to_cur_ans + dp[i - 1].prev_ind_ans {
                dp[i].prev_ind_ans = dp[i - 1].prev_ind_ans;
                dp[i].com_min = to_cur_min;
                dp[i].com_max = to_cur_max;
                dp[i].ind_ans = to_cur_ans;
            }

            if can_create_next {
                let mut new = Step::default();

                for j in i..(i + 3) {
                    new.com_min = new.com_min.min(a[j - 1].0);
                    new.com_max = new.com_max.max(a[j - 1].0);

                    new.ind_ans = new.com_max - new.com_min;
                }
                new.prev_ind_ans = dp[i - 1].ind_ans + dp[i - 1].prev_ind_ans;

                let new_maybe_ans = dp[i + 3 - 1].ind_ans + dp[i + 3 - 1].prev_ind_ans;
                if new_maybe_ans > new.ind_ans + new.prev_ind_ans {
                    dp[i + 3 - 1] = new;
                }
            }
        }

        let mut ans = vec![0; n];
        let mut cur = 1;
        let mut i = n;

        loop {
            let go = dp[i].com_min;

            while i > 0 && !(go == usize::MAX || go > a[i - 1].0) {
                ans[a[i - 1].1] = cur;
                i -= 1;
            }

            if i == 0 {
                break;
            } else {
                ans[a[i - 1].1] = cur;
            }
            cur += 1;
        }

        println!(
            "{x} {cnt}",
            x = dp[n].prev_ind_ans + dp[n].ind_ans,
            cnt = cur,
        );
        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
