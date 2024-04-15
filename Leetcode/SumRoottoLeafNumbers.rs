// https://leetcode.com/problems/sum-root-to-leaf-numbers

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
    fn sum_numbers_(root: Option<Rc<RefCell<TreeNode>>>, cur: i32) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            let cur = cur * 10 + root.val;

            if root.left.is_some() || root.right.is_some() {
                return Solution::sum_numbers_(root.left.clone(), cur)
                    + Solution::sum_numbers_(root.right.clone(), cur);
            } else {
                return cur;
            }
        }

        0
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::sum_numbers_(root, 0)
    }
}
