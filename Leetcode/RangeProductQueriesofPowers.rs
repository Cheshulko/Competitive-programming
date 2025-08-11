// https://leetcode.com/problems/range-product-queries-of-powers

struct Solution {}

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pows = vec![];
        let mut p = 1;
        while p <= n {
            if (p & n) > 0 {
                pows.push(p as usize);
            }
            p <<= 1;
        }

        fn mod_inverse(mut x: usize, m: usize) -> usize {
            let mut res = 1;
            let mut exp = m - 2;

            while exp != 0 {
                if exp % 2 == 1 {
                    res = res * x % m;
                }
                x = x * x % m;
                exp /= 2;
            }
            res
        }

        const MOD: usize = 1_000_000_000 + 7;
        let mut pref = vec![1];
        for p in pows.into_iter() {
            pref.push((p * pref.last().unwrap()) % MOD);
        }

        queries
            .into_iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize + 1;

                (pref[r] * mod_inverse(pref[l], MOD) % MOD) as i32
            })
            .collect()
    }
}
