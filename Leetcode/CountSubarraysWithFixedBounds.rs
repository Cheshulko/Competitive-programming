// https://leetcode.com/problems/count-subarrays-with-fixed-bounds

struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let n = nums.len();

        let mut segments = vec![];
        {
            let mut l = 0;
            let mut r = 0;
            while l < n {
                while l < n && (nums[l] < min_k || nums[l] > max_k) {
                    l += 1;
                }
                r = l + 1;
                while r < n && nums[r] >= min_k && nums[r] <= max_k {
                    r += 1;
                }
                if l < n {
                    // [l, r)
                    segments.push(&nums[l..r]);
                    l = r;
                }
            }
        }

        fn solve<'a>(segments: &'a [i32], min_k: i32, max_k: i32) -> usize {
            let mut mi_indx = vec![];
            for (i, &x) in segments.iter().enumerate() {
                if x == min_k {
                    mi_indx.push(i);
                }
            }

            let mut ma_indx = vec![];
            for (i, &x) in segments.iter().enumerate() {
                if x == max_k {
                    ma_indx.push(i);
                }
            }

            let n = segments.len();
            let mut ans = 0;
            for i in 0..n {
                let next_mi_p = mi_indx.partition_point(|&p| p < i);
                if next_mi_p == mi_indx.len() {
                    break;
                }
                let next_mi_ind = mi_indx[next_mi_p];

                let next_ma_p = ma_indx.partition_point(|&p| p < i);
                if next_ma_p == ma_indx.len() {
                    break;
                }

                let next_ma_ind = ma_indx[next_ma_p];

                let ind = next_mi_ind.max(next_ma_ind);

                ans += n - ind;
            }

            ans
        }

        segments
            .into_iter()
            .map(|segment| solve(segment, min_k, max_k) as i64)
            .sum()
    }
}
