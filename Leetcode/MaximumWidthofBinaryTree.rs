// https://leetcode.com/problems/maximum-width-of-binary-tree

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn bfs(cur: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: VecDeque<(Rc<RefCell<TreeNode>>, u32, u64)> = VecDeque::new();
        let mut levels: Vec<(u64, u64)> = Vec::new();
        let mut ans = 0;

        if let Some(cur) = cur.clone() {
            q.push_back((cur.clone(), 0, 1));

            while !q.is_empty() {
                let cur = q.pop_front().unwrap();
                let node = cur.0.borrow_mut();

                let level = cur.1 as usize;
                if levels.len() <= level {
                    levels.push((u64::MAX, u64::MIN));
                }
                levels[level].0 = std::cmp::min(levels[level].0, cur.2);
                levels[level].1 = std::cmp::max(levels[level].1, cur.2);
                ans = ans.max(levels[level].1 - levels[level].0 + 1);

                if let Some(to_left) = node.left.clone() {
                    q.push_back((to_left, cur.1 + 1, cur.2 * 2));
                }
                if let Some(to_right) = node.right.clone() {
                    q.push_back((to_right, cur.1 + 1, cur.2 * 2 + 1));
                }
            }
        }

        ans as i32
    }

    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::bfs(root)
    }
}
