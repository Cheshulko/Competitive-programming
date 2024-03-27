// https://leetcode.com/problems/subarray-product-less-than-k

struct Solution {}

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut i = 0;
        let mut j = 0;

        let mut ans = 0;
        let mut cnt = 0;
        let mut cur = 1;

        while i < n {
            while j < n && cur * nums[j] < k {
                cur *= nums[j];
                cnt += 1;
                j += 1;
            }

            if i != j {
                cur /= nums[i];
            }

            ans += cnt;
            cnt -= 1;
            cnt = cnt.max(0);

            i += 1;
            j = j.max(i);
        }

        ans
    }
}
