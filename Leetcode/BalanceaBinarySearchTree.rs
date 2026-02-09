// https://leetcode.com/problems/balance-a-binary-search-tree

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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs_collect(
            root: Option<Rc<RefCell<TreeNode>>>,
            values: &mut Vec<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(root) = root {
                let (left, right) = {
                    let root_ = root.borrow();
                    let left = root_.left.clone();
                    let right = root_.right.clone();

                    (left, right)
                };

                dfs_collect(left, values);
                {
                    let mut root_ = root.borrow_mut();
                    root_.left = None;
                    root_.right = None;
                    values.push(root.clone());
                }
                dfs_collect(right, values);
            }
        }

        fn dfs_build(values: &[Rc<RefCell<TreeNode>>]) -> Option<Rc<RefCell<TreeNode>>> {
            if values.is_empty() {
                return None;
            }

            let n = values.len();
            let m = n / 2;

            let root = values[m].clone();
            let left = dfs_build(&values[..m]);
            let right = dfs_build(&values[m + 1..]);
            {
                let mut root_ = root.borrow_mut();
                root_.left = left;
                root_.right = right;
            }

            Some(root)
        }

        let mut values = vec![];
        dfs_collect(root, &mut values);

        dfs_build(&values)
    }
}
