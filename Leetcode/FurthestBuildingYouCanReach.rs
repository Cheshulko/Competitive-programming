// https://leetcode.com/problems/furthest-building-you-can-reach

use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut heap = BinaryHeap::<i32>::new();
        let mut ans = 0;
        let mut cur = heights[0];

        fn use_bricks(cnt: i32, bricks: &mut i32, heap: &mut BinaryHeap<i32>) -> bool {
            if *bricks >= cnt {
                *bricks -= cnt;
                heap.push(cnt);

                true
            } else {
                false
            }
        }

        fn use_ladder(bricks: &mut i32, ladders: &mut i32, heap: &mut BinaryHeap<i32>) -> bool {
            if *ladders > 0 {
                *ladders -= 1;

                true
            } else {
                false
            }
        }

        for (ind, h) in heights.into_iter().enumerate() {
            if cur >= h {
                ans = ind;
                cur = h;
                continue;
            }

            let diff = h - cur;

            if use_bricks(diff, &mut bricks, &mut heap) {
                ans = ind;
                cur = h;
            } else if use_ladder(&mut bricks, &mut ladders, &mut heap) {
                if heap.peek().unwrap_or(&0) > &diff {
                    bricks += heap.pop().unwrap_or(0);

                    if use_bricks(diff, &mut bricks, &mut heap) {
                        ans = ind;
                        cur = h;
                    } else {
                        break;
                    }
                } else {
                    ans = ind;
                    cur = h;
                }
            } else {
                break;
            }
        }

        ans as i32
    }
}
