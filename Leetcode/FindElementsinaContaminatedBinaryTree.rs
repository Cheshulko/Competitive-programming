// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree

use std::cell::RefCell;
use std::rc::Rc;

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

struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }

    fn find(&self, mut target: i32) -> bool {
        let mut path = vec![];

        while target > 0 {
            path.push(target & 1);
            target = (target - 1) >> 1;
        }

        self.check(path, self.root.clone())
    }

    fn check(&self, mut path: Vec<i32>, cur_node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(cur_node) = cur_node {
            let cur_node = cur_node.borrow();

            match path.pop() {
                None => true,
                Some(1) => self.check(path, cur_node.left.clone()),
                Some(0) => self.check(path, cur_node.right.clone()),
                _ => unreachable!(),
            }
        } else {
            false
        }
    }
}
