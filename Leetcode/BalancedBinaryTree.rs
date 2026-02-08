// https://leetcode.com/problems/balanced-binary-tree

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<(bool, usize)> {
            if let Some(root) = root {
                let root = root.borrow();

                let left = dfs(root.left.clone()).unwrap_or((true, 0));
                let right = dfs(root.right.clone()).unwrap_or((true, 0));

                Some((
                    left.0 && right.0 && left.1.abs_diff(right.1) <= 1,
                    1 + left.1.max(right.1),
                ))
            } else {
                None
            }
        }

        dfs(root.clone()).map(|root| root.0).unwrap_or(true)
    }
}
