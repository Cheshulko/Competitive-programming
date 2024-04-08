// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch

struct Solution {}

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students.into_iter().fold([0; 2], |mut v, s| {
            v[s as usize] += 1;
            v
        });

        let cnt = sandwiches.len();
        for (i, sandwiche) in sandwiches.into_iter().enumerate() {
            let students = &mut students[sandwiche as usize];
            if students == &0 {
                return (cnt - i) as i32;
            } else {
                *students -= 1;
            }
        }

        0
    }
}
