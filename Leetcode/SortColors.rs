// https://leetcode.com/problems/sort-colors

struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut cnt = vec![0; 3];
        for &x in nums.iter() {
            cnt[x as usize] += 1;
        }
        for x in nums.iter_mut() {
            for t in 0..3 {
                if cnt[t] > 0 {
                    cnt[t] -= 1;
                    *x = t as i32;
                    break;
                }
            }
        }
    }
}
