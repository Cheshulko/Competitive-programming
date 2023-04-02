// https://leetcode.com/problems/successful-pairs-of-spells-and-potions

struct Solution {}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        fn upper_bound(nums: &Vec<i32>, target: i64) -> usize {
            let mut l = 0;
            let mut r = nums.len();

            while r - l > 1 {
                let mid = (l + r) / 2;
                if target < nums[mid] as i64 {
                    r = mid;
                } else {
                    l = mid;
                }
            }

            if nums[l] as i64 > target {
                l
            } else {
                r
            }
        }

        let mut potions = potions;
        potions.sort();

        let mut ans: Vec<i32> = Vec::new();

        for x in spells {
            let x = x as i64;
            let target = success / x + if success % x > 0 { 1 } else { 0 } - 1;
            let ind = upper_bound(&potions, target);
            let len = potions[ind..].len() as i32;
            ans.push(len);
        }

        ans
    }
}
