// https://leetcode.com/problems/evaluate-boolean-binary-tree

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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let root = root.borrow();

            let left = &root.left;
            let right = &root.right;

            if left.is_some() {
                let left = Solution::evaluate_tree(left.clone());
                let right = Solution::evaluate_tree(right.clone());

                if root.val == 2 {
                    left || right
                } else {
                    left && right
                }
            } else {
                if root.val == 1 {
                    true
                } else {
                    false
                }
            }
        } else {
            true
        }
    }
}
