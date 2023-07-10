// https://leetcode.com/problems/minimum-depth-of-binary-tree

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();

            let left = Solution::min_depth(root.left.clone());
            let right = Solution::min_depth(root.right.clone());

            match (left, right) {
                (0, 0) => 1,
                (0, r) => 1 + r,
                (l, 0) => 1 + l,
                _ => 1 + left.min(right),
            }
        } else {
            0
        }
    }
}
