// https://leetcode.com/problems/range-sum-of-bst

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        root.map(|root| {
            let root = root.borrow();
            (root.val >= low)
                .then_some(Solution::range_sum_bst(root.left.clone(), low, high))
                .unwrap_or(0)
                + (root.val <= high)
                    .then_some(Solution::range_sum_bst(root.right.clone(), low, high))
                    .unwrap_or(0)
                + root.val * (root.val >= low && root.val <= high) as i32
        })
        .unwrap_or(0)
    }
}
