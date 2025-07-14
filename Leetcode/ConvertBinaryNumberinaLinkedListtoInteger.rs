// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer

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
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut ans = 0;

        while let Some(cur) = head {
            ans <<= 1;
            ans += cur.val;

            head = cur.next;
        }

        ans
    }
}
