// https://leetcode.com/problems/non-overlapping-intervals

struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| match a[1] == b[1] {
            true => a[0].cmp(&b[0]),
            false => a[1].cmp(&b[1]),
        });
        let mut cnt = 0;
        let mut cur_end = i32::MIN;

        for int in intervals.into_iter() {
            if int[0] < cur_end {
                cnt += 1;
            } else {
                cur_end = int[1];
            }
        }

        cnt
    }
}
