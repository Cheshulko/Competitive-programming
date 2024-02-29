// https://leetcode.com/problems/even-odd-tree

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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut levels = vec![];
        Solution::go_go(root, 0, &mut levels)
    }

    fn go_go(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, levels: &mut Vec<i32>) -> bool {
        if let Some(node) = node {
            let node = node.borrow();

            let depth_t = (depth % 2) as i32;
            let node_t = node.val % 2;

            if depth_t == node_t {
                return false;
            }

            let res = if levels.len() == depth {
                levels.push(node.val);

                true
            } else {
                let is_level_ok = if depth % 2 == 1 {
                    levels[depth] > node.val
                } else {
                    levels[depth] < node.val
                };

                levels[depth] = node.val;
                is_level_ok
            };

            return res
                && Solution::go_go(node.left.clone(), depth + 1, levels)
                && Solution::go_go(node.right.clone(), depth + 1, levels);
        } else {
            return true;
        }
    }
}
