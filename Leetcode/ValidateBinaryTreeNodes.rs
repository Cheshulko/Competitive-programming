// https://leetcode.com/problems/validate-binary-tree-nodes

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;
        let mut used = vec![false; n];

        let mut deg = vec![0; n];
        for cur in 0..n {
            let left = left_child[cur];
            let right = right_child[cur];

            if left != -1 {
                deg[left as usize] += 1;
            }
            if right != -1 {
                deg[right as usize] += 1;
            }
        }
        let start = deg
            .iter()
            .enumerate()
            .filter(|(_, x)| x == &&0)
            .collect::<Vec<_>>();

        if start.len() != 1 {
            return false;
        }

        let start = start[0].0;

        let mut q = VecDeque::<usize>::new();
        q.push_back(start);
        used[start] = true;

        while let Some(cur) = q.pop_front() {
            let left = left_child[cur];
            let right = right_child[cur];

            if left != -1 {
                let left = left as usize;
                if used[left] {
                    return false;
                }
                used[left] = true;
                q.push_back(left);
            }
            if right != -1 {
                let right = right as usize;
                if used[right] {
                    return false;
                }
                used[right] = true;
                q.push_back(right);
            }
        }

        !used.iter().any(|x| !x)
    }
}
