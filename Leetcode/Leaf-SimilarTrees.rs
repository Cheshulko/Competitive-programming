// https://leetcode.com/problems/leaf-similar-trees

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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::leafs(root1) == Solution::leafs(root2)
    }

    fn leafs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];

        if let Some(root) = root {
            let root = root.borrow();
            let is_leaf = root.left.is_none() && root.right.is_none();

            if is_leaf {
                v.push(root.val);
            } else {
                v.extend(Solution::leafs(root.left.clone()));
                v.extend(Solution::leafs(root.right.clone()));
            }
        }

        v
    }
}
