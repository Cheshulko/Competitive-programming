// https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut dp = BTreeMap::<i32, i32>::new();

        obstacles
            .iter()
            .map(|obs| {
                let mut cur_value = 1;

                if let Some((_, value)) = dp.range(..=obs).next_back() {
                    cur_value += *value;
                }

                let range = (obs + 1)..;
                if let Some((k, v)) = dp.range(range).next().map(|(x, y)| (*x, *y)) {
                    if v <= cur_value {
                        dp.remove(&k);
                    }
                }

                dp.insert(*obs, cur_value);
                cur_value
            })
            .collect()
    }
}
