// https://leetcode.com/problems/asteroid-collision

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut s: VecDeque<i32> = VecDeque::new();
        for a in asteroids.into_iter() {
            let mut broken = false;
            while let Some(back) = s.back() {
                if a < 0 && back > &0 {
                    match (-a).cmp(back) {
                        std::cmp::Ordering::Less => {
                            broken = true;
                            break;
                        }
                        std::cmp::Ordering::Equal => {
                            s.pop_back();
                            broken = true;
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            s.pop_back();
                        }
                    }
                } else {
                    break;
                }
            }

            if !broken {
                s.push_back(a);
            }
        }

        s.into_iter().collect::<Vec<_>>()
    }
}
