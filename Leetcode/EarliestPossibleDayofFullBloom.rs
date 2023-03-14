// https://leetcode.com/problems/earliest-possible-day-of-full-bloom/description/

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut x: Vec<(&i32, &i32)> = plant_time.iter().zip(grow_time.iter()).collect();

        x.sort_by(|&a, &b| {
            let or = b.1.cmp(&a.1);
            if let std::cmp::Ordering::Equal = or {
                return a.0.cmp(&b.0);
            } else {
                return or;
            }
        });
        let mut t = 0;
        let mut ans = 0;
        for cur in &x {
            t += cur.0;
            ans = std::cmp::max(ans, t + cur.1);
        }

        ans
    }
}
