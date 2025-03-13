// https://leetcode.com/problems/zero-array-transformation-ii

struct Solution {}

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        fn check(k: usize, nums: &Vec<i32>, queries: &Vec<Vec<i32>>, pref: &mut Vec<i64>) -> bool {
            for x in pref.iter_mut() {
                *x = 0;
            }

            for i in 0..k {
                let l = queries[i][0] as usize;
                let r = queries[i][1] as usize;
                let v = queries[i][2] as i64;
                pref[l] += v;
                pref[r + 1] -= v;
            }

            for i in 0..nums.len() {
                pref[i + 1] += pref[i];
            }

            for i in 0..nums.len() {
                if pref[i] < nums[i] as i64 {
                    return false;
                }
            }

            return true;
        }

        let mut pref = vec![0; nums.len() + 1];

        if !check(queries.len(), &nums, &queries, &mut pref) {
            return -1;
        }

        if nums.iter().all(|&x| x == 0) {
            return 0;
        }

        let mut l = 0;
        let mut r = queries.len();
        while r - l > 1 {
            let m = (l + r) >> 1;

            if check(m, &nums, &queries, &mut pref) {
                r = m;
            } else {
                l = m;
            }
        }

        r as i32
    }
}
