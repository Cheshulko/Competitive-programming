// https://leetcode.com/problems/hand-of-straights

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        let group_size = group_size as usize;

        if n % group_size != 0 {
            return false;
        }

        let mut cnt = BTreeMap::<usize, usize>::new();
        for h in hand.into_iter() {
            *cnt.entry(h as usize).or_default() += 1;
        }

        let mut cnt = cnt.into_iter().collect::<Vec<_>>();
        let n = cnt.len();

        let mut can = true;
        for i in 0..=(n - n.min(group_size)) {
            if cnt[i].1 > 0 {
                let (val, cur) = cnt[i];
                for j in i..(i + group_size) {
                    if j >= cnt.len() || cnt[j].1 < cur || cnt[j].0 != val + j - i {
                        can = false;
                        break;
                    } else {
                        cnt[j].1 -= cur;
                    }
                }
            }
        }

        can && cnt.into_iter().all(|x| x.1 == 0)
    }
}
