// https://leetcode.com/problems/maximum-difference-by-remapping-a-digit

struct Solution {}

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let mut mi = i32::MAX;
        let mut ma = i32::MIN;

        for d in 0..10 {
            let mut num2 = num;
            let mut p = 1;

            let mut num0 = 0;
            let mut num9 = 0;

            while num2 > 0 {
                let dd = num2 % 10;
                if dd == d {
                    num0 += 0 * p;
                    num9 += 9 * p;
                } else {
                    num0 += dd * p;
                    num9 += dd * p;
                }

                num2 /= 10;
                p *= 10;
            }

            mi = mi.min(num0);
            ma = ma.max(num9);
        }

        ma - mi
    }
}
