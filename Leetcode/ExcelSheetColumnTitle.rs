// https://leetcode.com/problems/excel-sheet-column-title

struct Solution {}

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut r = vec![];
        while column_number > 0 {
            let up = column_number % 26;
            column_number /= 26;
            if up > 0 {
                r.push(((64 + up) as u8) as char);
            } else {
                column_number -= 1;
                r.push('Z');
            }
        }
        r.iter().rev().collect::<_>()
    }
}
