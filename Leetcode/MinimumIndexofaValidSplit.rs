// https://leetcode.com/problems/minimum-index-of-a-valid-split

struct Solution {}

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let dominant = {
            let mut s = vec![];

            for &num in nums.iter() {
                if let Some(&l) = s.last() {
                    if l == num {
                        s.push(l);
                    } else {
                        s.pop();
                    }
                } else {
                    s.push(num);
                }
            }

            s[0]
        };

        let dominant_cnt = nums.iter().filter(|&x| *x == dominant).count();

        let n = nums.len();
        let mut left_cnt = 0;
        for i in 0..n {
            left_cnt += (nums[i] == dominant) as usize;
            let right_cnt = dominant_cnt - left_cnt;

            let left_len = i + 1;
            let right_len = n - left_len;

            if left_cnt > left_len / 2 && right_cnt > right_len / 2 {
                return i as i32;
            }
        }

        -1
    }
}
