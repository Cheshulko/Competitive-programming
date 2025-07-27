// https://leetcode.com/problems/count-hills-and-valleys-in-an-array

struct Solution {}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let compress = nums
            .into_iter()
            .scan(-1, |state, num| {
                (*state != num)
                    .then(|| {
                        *state = num;

                        Some(num)
                    })
                    .or_else(|| Some(None))
            })
            .flatten()
            .collect::<Vec<_>>();

        let mut ans = 0;
        for range in compress.windows(3) {
            if let [prev, cur, next] = range {
                let hill = prev < cur && cur > next;
                let valley = prev > cur && cur < next;

                ans += (hill || valley) as i32;
            }
        }

        ans
    }
}
