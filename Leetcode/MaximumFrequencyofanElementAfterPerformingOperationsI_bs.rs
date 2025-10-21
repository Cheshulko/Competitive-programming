// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i

struct Solution {}

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort_unstable();

        let num_operations = num_operations as usize;
        let n = nums.len();
        let mi = nums[0];
        let ma = nums[n - 1];

        let mut freq = vec![0; ma as usize + 1];
        for &num in nums.iter() {
            freq[num as usize] += 1;
        }

        let mut ans = 0;
        for i in mi..=ma {
            let l = nums.partition_point(|&num| num + k < i);
            let r = nums.partition_point(|&num| num - k <= i);
            let cnt = r - l - freq[i as usize];

            ans = ans.max(freq[i as usize] + cnt.min(num_operations));
        }

        ans as i32
    }
}
