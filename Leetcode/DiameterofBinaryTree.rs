// https://leetcode.com/problems/diameter-of-binary-tree

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::depth_and_diameter(root).1 - 1
    }

    fn depth_and_diameter(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root) = root {
            let root = root.borrow();

            let (l_depth, l_diameter) = Solution::depth_and_diameter(root.left.clone());
            let (r_depth, r_diameter) = Solution::depth_and_diameter(root.right.clone());

            (
                1 + l_depth.max(r_depth),
                l_diameter.max(r_diameter).max(1 + l_depth + r_depth),
            )
        } else {
            (0, 0)
        }
    }
}
