// https://leetcode.com/problems/boats-to-save-people

struct Solution {}

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();
        let mut ans: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = people.len() as i32 - 1;

        while i < j {
            if people[i as usize] + people[j as usize] > limit {
                j -= 1;
            } else {
                j -= 1;
                i += 1;
            }
            ans += 1;
        }
        if i == j {
            ans += 1;
        }

        ans
    }
}
