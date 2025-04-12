// https://leetcode.com/problems/find-the-count-of-good-integers

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        fn build_valid_palindroms(
            num: usize,
            ind: usize,
            k: usize,
            pows: &[usize],
            palindroms: &mut HashSet<usize>,
        ) {
            let n = pows.len();
            let st = (ind == 0) as usize;

            let (mid, even) = ((n - 1) / 2, (1 - n & 1));

            if ind == mid {
                for j in st..=9 {
                    let num2 = num + j * pows[ind] + even * j * pows[n - 1 - ind];
                    if num2 % k == 0 {
                        palindroms.insert(num2);
                    }
                }
            } else {
                for j in st..=9 {
                    let num2 = num + j * pows[ind] + j * pows[n - 1 - ind];
                    build_valid_palindroms(num2, ind + 1, k, pows, palindroms);
                }
            }
        }

        fn count_numbers(
            ind: usize,
            mut mask: [usize; 10],
            dp: &mut HashMap<[usize; 10], usize>,
        ) -> usize {
            if let Some(&ans) = dp.get(&mask) {
                return ans;
            }

            let st = (ind == 0) as usize;
            let mut ans = 0;
            for i in st..=9 {
                if mask[i] > 0 {
                    mask[i] -= 1;
                    ans += count_numbers(ind + 1, mask, dp);
                    mask[i] += 1;
                }
            }

            dp.insert(mask.clone(), ans);
            ans
        }

        let n = n as usize;
        let k = k as usize;

        let pows = (1..=n)
            .scan(1, |pow, _| {
                let v = *pow;
                *pow *= 10;

                Some(v)
            })
            .collect::<Vec<_>>();

        let mut palindroms = HashSet::new();
        build_valid_palindroms(0, 0, k, &pows, &mut palindroms);

        let mut masks = HashSet::new();
        for mut num in palindroms.into_iter() {
            let mut cnt = [0; 10];
            while num > 0 {
                cnt[num % 10] += 1;
                num /= 10;
            }

            masks.insert(cnt);
        }

        let mut dp = HashMap::new();
        dp.insert([0; 10], 1);

        let mut ans = 0;
        for mask in masks.into_iter() {
            ans += count_numbers(0, mask, &mut dp);
        }

        ans as i64
    }
}
