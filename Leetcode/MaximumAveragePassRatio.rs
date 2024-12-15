// https://leetcode.com/problems/maximum-average-pass-ratio

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut bh = BinaryHeap::<(usize, usize)>::new();

        fn get_delta(class: &Vec<i32>) -> usize {
            let pass = class[0] as usize;
            let total = class[1] as usize;

            let num = 1_000_000_000 * ((pass + 1) * total - pass * (total + 1));
            let den = (total + 1) * total;

            return num / den;
        }

        for (i, class) in classes.iter().enumerate() {
            bh.push((get_delta(class), i));
        }

        for _ in 0..extra_students {
            if let Some((_, i)) = bh.pop() {
                classes[i][0] += 1;
                classes[i][1] += 1;

                bh.push((get_delta(&classes[i]), i));
            }
        }

        let n = classes.len() as f64;

        classes
            .into_iter()
            .map(|class| class[0] as f64 / class[1] as f64)
            .sum::<f64>()
            / n
    }
}
