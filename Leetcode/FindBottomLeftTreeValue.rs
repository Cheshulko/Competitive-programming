// https://leetcode.com/problems/find-bottom-left-tree-value

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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::min_left_on_depth(root).0.unwrap()
    }

    fn min_left_on_depth(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<i32>, usize) {
        if let Some(root) = root {
            let root = root.borrow();

            let left @ (_, l_dep) = Solution::min_left_on_depth(root.left.clone());
            let right @ (_, r_dep) = Solution::min_left_on_depth(root.right.clone());

            let (min, depth) = match l_dep.cmp(&r_dep) {
                std::cmp::Ordering::Less => right,
                _ => left,
            };

            (Some(min.unwrap_or(root.val)), depth + 1)
        } else {
            (None, usize::MIN)
        }
    }
}
