// https://leetcode.com/problems/middle-of-the-linked-list

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(head_) = &head {
            let mut fast = head_.next.clone();
            let mut slow = head;

            while let (Some(slow_), Some(fast_)) = (&slow, &fast) {
                slow = slow_.next.clone();
                fast = fast_.next.clone();

                if let Some(fast_) = fast {
                    fast = fast_.next;
                }
            }

            slow
        } else {
            None
        }
    }
}
