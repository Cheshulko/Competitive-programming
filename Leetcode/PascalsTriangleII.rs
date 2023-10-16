// https://leetcode.com/problems/pascals-triangle-ii

struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut cur = vec![1];

        (0..row_index).for_each(|_| {
            cur = cur.iter().enumerate().fold(vec![1], |mut next, (j, x)| {
                next.push(x + cur.get(j + 1).unwrap_or(&0));
                next
            });
        });

        cur
    }
}
