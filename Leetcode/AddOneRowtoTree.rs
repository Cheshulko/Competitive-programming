// https://leetcode.com/problems/add-one-row-to-tree

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

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

impl Solution {
    fn add_one_row_(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, cur_depth: i32) {
        if let Some(ref mut root_) = root {
            if cur_depth + 1 == depth {
                let root_ = root_.borrow_mut();

                let left = root_.borrow().left.clone();
                let right = root_.borrow().right.clone();

                let mut left_ = TreeNode::new(val);
                left_.left = left;
                root_.as_ref().borrow_mut().left = Some(Rc::new(RefCell::new(left_)));

                let mut right_ = TreeNode::new(val);
                right_.right = right;
                root_.as_ref().borrow_mut().right = Some(Rc::new(RefCell::new(right_)));

                return;
            } else {
                let root_ = root_.borrow();
                Solution::add_one_row_(root_.left.clone(), val, depth, cur_depth + 1);
                Solution::add_one_row_(root_.right.clone(), val, depth, cur_depth + 1);
            }
        }
    }

    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut root_new = TreeNode::new(val);
            root_new.left = root;
            Some(Rc::new(RefCell::new(root_new)))
        } else {
            Solution::add_one_row_(root.clone(), val, depth, 1);
            root
        }
    }
}
