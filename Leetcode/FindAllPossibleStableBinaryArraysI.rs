// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-i

struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let [zeros, ones, limit] = [zero as usize, one as usize, limit as usize];
        let n = zeros + ones;

        let mut dp_ones = vec![vec![[0_usize; 201]; ones + 1]; n + 1];
        let mut dp_zeros = vec![vec![[0_usize; 201]; zeros + 1]; n + 1];

        dp_ones[0][0][0] = 1;
        dp_zeros[0][0][0] = 1;

        for len in 1..=n {
            for used_ones in 0..=len.min(ones) {
                let have_ones = ones - used_ones;

                let used_zeros = len - 1 - used_ones;
                if used_zeros > zeros {
                    continue;
                }
                let have_zeros = zeros - used_zeros;

                if have_ones > 0 {
                    for tail_ones in 0..limit {
                        dp_ones[len][used_ones + 1][tail_ones + 1] +=
                            dp_ones[len - 1][used_ones][tail_ones];

                        dp_ones[len][used_ones + 1][tail_ones + 1] %= M;
                    }

                    for tail_zeros in 1..=limit {
                        dp_ones[len][used_ones + 1][1] += dp_zeros[len - 1][used_zeros][tail_zeros];
                        dp_ones[len][used_ones + 1][1] %= M;
                    }
                }

                if have_zeros > 0 {
                    for tail_zeros in 0..limit {
                        dp_zeros[len][used_zeros + 1][tail_zeros + 1] +=
                            dp_zeros[len - 1][used_zeros][tail_zeros];

                        dp_zeros[len][used_zeros + 1][tail_zeros + 1] %= M;
                    }

                    for tail_ones in 1..=limit {
                        dp_zeros[len][used_zeros + 1][1] += dp_ones[len - 1][used_ones][tail_ones];
                        dp_zeros[len][used_zeros + 1][1] %= M;
                    }
                }
            }
        }

        let mut ans = 0;
        for lim in 0..=limit {
            ans += dp_ones[n][ones][lim];
            ans += dp_zeros[n][zeros][lim];
            ans %= M;
        }

        ans as i32
    }
}
