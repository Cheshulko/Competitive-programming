// https://leetcode.com/problems/ipo

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let k = k as usize;

        let mut can_cap = BinaryHeap::<i32>::new();

        let mut cap_pro = capital
            .into_iter()
            .zip(profits.into_iter().map(|x| Reverse(x)))
            .collect::<Vec<_>>();

        cap_pro.sort_unstable();

        let mut pre_ind = 0;
        for _ in 0..k {
            let to_add_ind = cap_pro.partition_point(|x| x.0 <= w);
            for j in pre_ind..to_add_ind {
                can_cap.push(cap_pro[j].1 .0);
            }
            pre_ind = to_add_ind;

            if let Some(take) = can_cap.pop() {
                w += take;
            } else {
                break;
            }
        }

        w
    }
}
