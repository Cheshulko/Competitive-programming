// https://leetcode.com/problems/minimum-time-to-repair-cars

struct Solution {}

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as i64;

        fn can(ranks: &Vec<i32>, cars: i64, time: i64) -> bool {
            let mut cnt = 0;
            for &rank in ranks.iter() {
                let rank = rank as i64;
                let n2 = time / rank;
                let n = n2.isqrt();
                cnt += n;
            }

            cnt >= cars
        }

        let mut l: i64 = 0;
        let mut r: i64 = 1 << 60;

        while r - l > 1 {
            let m = (l + r) >> 1;
            if can(&ranks, cars, m) {
                r = m;
            } else {
                l = m;
            }
        }

        r
    }
}
