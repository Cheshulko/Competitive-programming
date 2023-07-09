impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = -1;

        for st in 0..nums.len() {
            let mut l = 1;
            let mut cur = nums[st];
            let mut need = 1;

            for i in (st + 1)..nums.len() {
                if nums[i] - cur == need {
                    l += 1;
                    need *= -1;
                    ans = ans.max(l);
                } else {
                    l = 1;
                    need = 1;
                }

                cur = nums[i];
            }
        }

        ans
    }
}
