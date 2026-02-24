// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers

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

struct Solution;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn go(root: Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
            if let Some(root) = root {
                let root = root.borrow();
                let num = (num << 1) + root.val;

                if root.left.is_none() && root.right.is_none() {
                    num
                } else {
                    let left = go(root.left.clone(), num);
                    let right = go(root.right.clone(), num);

                    left + right
                }
            } else {
                0
            }
        }

        go(root, 0)
    }
}
