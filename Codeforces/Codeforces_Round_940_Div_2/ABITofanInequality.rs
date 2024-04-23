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

    const BITS: usize = 32;

    fn left_bit(mut x: i64) -> i64 {
        let mut cnt = 0;
        while x > 0 {
            x >>= 1;
            cnt += 1;
        }
        cnt - 1
    }

    let mut pref = [[[0; 2]; BITS]; 100001];
    let mut suf = [[[0; 2]; BITS]; 100001];

    for _ in 0..t {
        let n = cin.next::<usize>();

        let mut a = vec![];

        for _ in 0..n {
            let x = cin.next::<i64>();
            a.push(x);
        }

        for bit in 0..BITS {
            let x = a[0];
            let y = a[n - 1];

            pref[0][bit][0] = ((1 << bit) & x == 0) as i64;
            pref[0][bit][1] = ((1 << bit) & x != 0) as i64;

            suf[n - 1][bit][0] = ((1 << bit) & y == 0) as i64;
            suf[n - 1][bit][1] = ((1 << bit) & y != 0) as i64;
        }

        for i in 1..n {
            let x = a[i];

            for bit in 0..BITS {
                let has_bit = ((1 << bit) & x != 0) as i64;
                if has_bit > 0 {
                    pref[i][bit][1] = pref[i - 1][bit][0] + 1;
                    pref[i][bit][0] = pref[i - 1][bit][1];
                } else {
                    pref[i][bit][1] = pref[i - 1][bit][1];
                    pref[i][bit][0] = pref[i - 1][bit][0] + 1;
                }
            }
        }

        for i in (0..(n - 1)).rev() {
            let x = a[i];

            for bit in 0..BITS {
                let has_bit = ((1 << bit) & x != 0) as i64;
                if has_bit > 0 {
                    suf[i][bit][1] = suf[i + 1][bit][0] + 1;
                    suf[i][bit][0] = suf[i + 1][bit][1];
                } else {
                    suf[i][bit][1] = suf[i + 1][bit][1];
                    suf[i][bit][0] = suf[i + 1][bit][0] + 1;
                }
            }
        }

        let mut ans = 0;

        for (pos, x) in a.into_iter().enumerate() {
            let left_bit_x = left_bit(x) as usize;

            let ans_p = pref[pos][left_bit_x][0];
            let ans_s = suf[pos][left_bit_x][0];

            let mut ans_u = 0;
            if pos > 0 && pos < n - 1 {
                ans_u += pref[pos - 1][left_bit_x][0] * suf[pos + 1][left_bit_x][1];
                ans_u += pref[pos - 1][left_bit_x][1] * suf[pos + 1][left_bit_x][0];
            }

            ans += ans_p + ans_s + ans_u;
        }

        println!("{ans}");
    }
}
