// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();

        let mut slow = 0;
        let mut fast = 0;

        while fast < n {
            if (slow > 1 && nums[slow - 1] > nums[slow])
                || (slow >= 2 && nums[slow - 2] == nums[slow])
            {
                while fast < n && nums[fast] <= nums[slow] {
                    fast += 1;
                }
                if fast == n {
                    break;
                }

                nums.swap(slow, fast);
                continue;
            }

            slow += 1;
            fast += 1;
        }

        slow as i32
    }
}
