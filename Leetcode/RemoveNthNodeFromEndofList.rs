// https://leetcode.com/problems/remove-nth-node-from-end-of-list

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        Solution::tail(head, &mut n)
    }

    fn tail(cur: Option<Box<ListNode>>, n: &mut i32) -> Option<Box<ListNode>> {
        if let Some(mut cur_) = cur {
            cur_.next = Solution::tail(cur_.next, n);
            *n -= 1;

            if n == &0 {
                cur_.next
            } else {
                Some(cur_)
            }
        } else {
            cur
        }
    }
}
