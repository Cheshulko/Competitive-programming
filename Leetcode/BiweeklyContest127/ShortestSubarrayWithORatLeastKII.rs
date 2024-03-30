// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii

struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut ans = usize::MAX;

        let mut i = 0;
        let mut j = 0;

        let mut cnt = 0;

        let mut bits_cnt = vec![0; 32];

        fn add_bits(mut x: i64, v: &mut Vec<i64>) {
            let mut cnt = 0;
            while x > 0 {
                v[cnt] += x & 1;
                x >>= 1;
                cnt += 1;
            }
        }

        fn remove_bits(mut x: i64, v: &mut Vec<i64>) {
            let mut cnt = 0;
            while x > 0 {
                v[cnt] -= x & 1;
                x >>= 1;
                cnt += 1;
            }
        }

        fn get_num(v: &Vec<i64>) -> i64 {
            let mut cnt = 1;
            let mut r = 0;

            for i in 0..32 {
                r += cnt * ((v[i] != 0) as i64);
                cnt *= 2;
            }

            r
        }

        while i < n {
            while j < n {
                if get_num(&bits_cnt) >= k {
                    break;
                } else {
                    add_bits(nums[j] as i64, &mut bits_cnt);
                    j += 1;
                }
            }
            if get_num(&bits_cnt) >= k {
                if i == j {
                    ans = 1;
                } else {
                    ans = ans.min(j - i);
                }
            }

            if i != j {
                remove_bits(nums[i] as i64, &mut bits_cnt);
            }

            i += 1;
            j = j.max(i);
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
