// https://leetcode.com/problems/total-characters-in-string-after-transformations-i

struct Solution {}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: usize = 1_000_000_000 + 7;

        let t = t as usize;
        let s = s
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        let mut int = [false; 26];
        for &c in s.iter() {
            int[c] = true;
        }

        let mut mem = [0; 26];
        for c in 0..26 {
            if !int[c] {
                continue;
            }

            let mut tt = t;
            let mut calc = [0; 26];
            calc[c] = 1;

            while tt > 0 {
                let mut ma = 0;
                for cc in 0..26 {
                    if calc[cc] > 0 {
                        ma = cc;
                    }
                }

                let mut calc2 = [0; 26];
                let dtt = if ma == 25
                /* z */
                {
                    calc2[0] += calc[25];
                    calc2[1] += calc[25];
                    1
                } else {
                    (25 - ma).min(tt)
                };

                for cc in 0..(26 - dtt) {
                    calc2[cc + dtt] += calc[cc];
                }
                tt -= dtt;

                for cc in 0..26 {
                    calc[cc] = calc2[cc] % MOD;
                }
            }

            for cc in 0..26 {
                mem[c] += calc[cc];
                mem[c] %= MOD;
            }
        }

        s.into_iter().fold(0, |acc, d| (acc + mem[d]) % MOD) as i32
    }
}
