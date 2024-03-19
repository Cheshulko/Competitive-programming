// https://leetcode.com/problems/task-scheduler

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut heap = BinaryHeap::<(Reverse<i32>, i32, char)>::new();

        let frequencies = tasks
            .into_iter()
            .fold(HashMap::<char, i32>::new(), |mut hm, c| {
                *hm.entry(c).or_insert(0) += 1;
                hm
            });

        for (c, frequency) in frequencies.into_iter() {
            heap.push((Reverse(0), frequency, c));
        }

        let mut cur = 0;
        while let Some((Reverse(time), cnt, c)) = heap.pop() {
            cur = cur.max(time);
            if cnt > 1 {
                heap.push((Reverse(cur + n + 1), cnt - 1, c));
            }

            while let Some((Reverse(t_), _, _)) = heap.peek() {
                if *t_ > cur {
                    break;
                } else {
                    let (_, cnt_, c_) = heap.pop().unwrap();
                    heap.push((Reverse(cur + 1), cnt_, c_));
                }
            }

            cur += 1;
        }

        cur
    }
}
