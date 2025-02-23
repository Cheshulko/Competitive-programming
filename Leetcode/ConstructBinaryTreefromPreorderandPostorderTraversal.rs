// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal

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
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let n = preorder.len();
        let mut order: Vec<Rc<RefCell<TreeNode>>> = vec![];

        let mut i = 0;
        let mut j = 0;
        while i < n && j < n {
            if order.len() > 1 && order.last().unwrap().borrow().val == postorder[j] {
                let cur = order.pop();
                let prev = order.last().unwrap().clone();
                let mut prev = prev.borrow_mut();

                if prev.left.is_none() {
                    prev.left = cur;
                } else {
                    prev.right = cur;
                }

                j += 1;
            } else {
                order.push(Rc::new(RefCell::new(TreeNode::new(preorder[i]))));
                i += 1;
            }
        }

        while order.len() > 1 {
            let cur = order.pop().unwrap();
            let prev = order.last().unwrap().clone();
            let mut prev = prev.borrow_mut();

            if prev.left.is_none() {
                prev.left = Some(cur);
            } else {
                prev.right = Some(cur);
            }
        }

        assert!(order.len() > 0);

        order.pop()
    }
}
