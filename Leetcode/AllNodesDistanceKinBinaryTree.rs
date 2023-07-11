// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree

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
use std::collections::HashMap;
use std::rc::Rc;

mod cm {
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    use crate::TreeNode;

    // result enum
    #[derive(Debug, PartialEq, Eq)]
    pub enum LcaRes {
        NotFound,
        FoundP,
        FoundQ,
        FoundBoth(Rc<RefCell<TreeNode>>),
    }

    pub struct LCA;

    impl LCA {
        pub fn find(root: Rc<RefCell<TreeNode>>, p: Rc<RefCell<TreeNode>>) -> bool {
            let root = root.borrow();

            if root.val == p.borrow().val {
                return true;
            }

            match (&root.left, &root.right) {
                (None, None) => false,
                (Some(l), None) => LCA::find(l.clone(), p),
                (None, Some(r)) => LCA::find(r.clone(), p),
                (Some(l), Some(r)) => LCA::find(l.clone(), p.clone()) || LCA::find(r.clone(), p),
            }
        }
        pub fn lca_aux(
            root: Rc<RefCell<TreeNode>>,
            p: Rc<RefCell<TreeNode>>,
            q: Rc<RefCell<TreeNode>>,
        ) -> LcaRes {
            if root.borrow().val == p.borrow().val {
                if LCA::find(root.clone(), q.clone()) {
                    return LcaRes::FoundBoth(root.clone());
                } else {
                    return LcaRes::FoundP;
                }
            }

            if root.borrow().val == q.borrow().val {
                if LCA::find(root.clone(), p.clone()) {
                    return LcaRes::FoundBoth(root.clone());
                } else {
                    return LcaRes::FoundQ;
                }
            }

            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => LcaRes::NotFound,
                (Some(l), None) => LCA::lca_aux(l.clone(), p, q),
                (None, Some(r)) => LCA::lca_aux(r.clone(), p, q),
                (Some(l), Some(r)) => {
                    let result_l = LCA::lca_aux(l.clone(), p.clone(), q.clone());
                    match result_l {
                        LcaRes::FoundBoth(_) => result_l,
                        LcaRes::NotFound => LCA::lca_aux(r.clone(), p, q),
                        LcaRes::FoundP => {
                            if LCA::find(r.clone(), q.clone()) {
                                LcaRes::FoundBoth(root.clone())
                            } else {
                                LcaRes::FoundP
                            }
                        }
                        LcaRes::FoundQ => {
                            if LCA::find(r.clone(), p.clone()) {
                                LcaRes::FoundBoth(root.clone())
                            } else {
                                LcaRes::FoundQ
                            }
                        }
                    }
                }
            }
        }
        pub fn lowest_common_ancestor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match (root, p, q) {
                (Some(root), Some(p), Some(q)) => match LCA::lca_aux(root.clone(), p, q) {
                    LcaRes::FoundBoth(lca) => Some(lca),
                    _ => None,
                },
                _ => None,
            }
        }
    }
}

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: i32, depths: &mut HashMap<i32, i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            depths.insert(root.val, depth);

            Solution::dfs(root.left.clone(), depth + 1, depths);
            Solution::dfs(root.right.clone(), depth + 1, depths);
        }
    }

    fn calc(
        root: Option<Rc<RefCell<TreeNode>>>,
        cur: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        depths: &HashMap<i32, i32>,
        k: i32,
        ans: &mut Vec<i32>,
    ) {
        if target.is_some() && cur.is_some() {
            let lca = cm::LCA::lowest_common_ancestor(root.clone(), cur.clone(), target.clone());

            let lca_v = lca.clone().unwrap().borrow().val;
            let target_v = target.clone().unwrap().borrow().val;
            let cur_v = cur.clone().unwrap().borrow().val;

            let lca_d = depths.get(&lca_v).unwrap();
            let target_d = depths.get(&target_v).unwrap();
            let cur_d = depths.get(&cur_v).unwrap();

            if cur_d + target_d - 2 * lca_d == k {
                ans.push(cur_v);
            }

            let cur = cur.clone().unwrap();
            Solution::calc(
                root.clone(),
                cur.borrow().left.clone(),
                target.clone(),
                depths,
                k,
                ans,
            );

            Solution::calc(
                root.clone(),
                cur.borrow().right.clone(),
                target.clone(),
                depths,
                k,
                ans,
            );
        }
    }

    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        if root.is_some() {
            let mut depths = HashMap::<i32, i32>::new();
            Solution::dfs(root.clone(), 0, &mut depths);
            Solution::calc(root.clone(), root.clone(), target, &depths, k, &mut ans);
        }

        ans
    }
}
