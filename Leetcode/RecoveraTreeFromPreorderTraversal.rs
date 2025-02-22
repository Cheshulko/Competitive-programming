// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal

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
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut traversal = traversal.chars().collect::<VecDeque<_>>();

        fn get_next(traversal: &mut VecDeque<char>) -> (usize, i32) {
            let mut level = 0;
            while let Some(&next) = traversal.front() {
                if next == '-' {
                    level += 1;
                    traversal.pop_front();
                } else {
                    break;
                }
            }

            let mut num = 0;
            while let Some(&next) = traversal.front() {
                if next != '-' {
                    num = num * 10 + (next as u8 - b'0') as i32;
                    traversal.pop_front();
                } else {
                    break;
                }
            }

            (level, num)
        }

        let mut levels = vec![vec![]; 1001];
        let (root_level, root_val) = get_next(&mut traversal);
        assert!(root_level == 0);

        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        levels[0].push(root.clone());

        let mut cur_level = root_level;

        loop {
            let (next_level, next_value) = get_next(&mut traversal);
            match next_level.cmp(&cur_level) {
                Ordering::Less => {
                    while cur_level > next_level {
                        let (right, left) = if levels[cur_level].len() == 2 {
                            (levels[cur_level].pop(), levels[cur_level].pop())
                        } else {
                            (None, levels[cur_level].pop())
                        };

                        if right.is_none() && left.is_none() {
                            cur_level -= 1;

                            continue;
                        }

                        if let Some(par) = levels[cur_level - 1].last().clone() {
                            let mut par = par.borrow_mut();
                            par.left = left;
                            par.right = right;
                            cur_level -= 1;
                        } else {
                            cur_level -= 1;
                        }
                    }
                }
                _ => {}
            }

            levels[next_level].push(Rc::new(RefCell::new(TreeNode::new(next_value))));
            cur_level = next_level;
            if cur_level == 0 {
                break;
            }
        }

        Some(root)
    }
}
