// https://leetcode.com/problems/apply-operations-to-maximize-score

struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        pub fn bin_pow(mut a: usize, mut b: usize, m: usize) -> usize {
            let mut res = 1;
            while b > 0 {
                if b & 1 == 1 {
                    res = res * a % m;
                }
                a = a * a % m;
                b >>= 1;
            }
            res
        }

        let n = nums.len();
        let mut k = k as i64;
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();

        let mut values = vec![];
        for &(mut num) in nums.iter() {
            let mut cur = 0;
            let mut i = 2;
            while i * i <= num {
                if num % i == 0 {
                    cur += 1;
                    while num % i == 0 {
                        num /= i;
                    }
                }
                i += 1;
            }
            if num > 1 {
                cur += 1;
            }
            values.push(cur);
        }

        let mut lefts = vec![0; n];
        {
            let mut mon = vec![];
            mon.push((usize::MAX, -1));

            for i in 0..n {
                while let Some(last) = mon.last() {
                    if last.0 < values[i] {
                        mon.pop();
                    } else {
                        break;
                    }
                }
                assert!(!mon.is_empty());
                lefts[i] = mon.last().unwrap().1;

                mon.push((values[i], i as i64));
            }
        }

        let mut rights = vec![0; n];
        {
            let mut mon = vec![];
            mon.push((usize::MAX, n as i64));

            for i in (0..n).rev() {
                while let Some(last) = mon.last() {
                    if last.0 <= values[i] {
                        mon.pop();
                    } else {
                        break;
                    }
                }
                assert!(!mon.is_empty());
                rights[i] = mon.last().unwrap().1;

                mon.push((values[i], i as i64));
            }
        }

        let ma = nums.iter().max().copied().unwrap();
        let mut ranges = vec![0; ma + 1];
        for i in 0..n {
            let left = i as i64 - lefts[i];
            let right = rights[i] - i as i64;
            ranges[nums[i]] += left * right;
            ranges[nums[i]] = k.min(ranges[nums[i]]);
        }

        const MOD: usize = 1_000_000_000 + 7;
        let mut ans = 1;
        for x in (1..=ma).rev() {
            let have = ranges[x];
            let take = have.min(k);
            k -= take;

            ans *= bin_pow(x, take as usize, MOD);
            ans %= MOD;

            if k == 0 {
                break;
            }
        }

        ans as i32
    }
}
