// https://leetcode.com/problems/reverse-linked-list

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
   public:
    ListNode* reverseList(ListNode* head) {
        if (!head) {
            return nullptr;
        }

        auto next = head->next;
        if (next) {
            auto new_head = Solution::reverseList(next);
            next->next = head;
            head->next = nullptr;

            return new_head;
        } else {
            return head;
        }
    }
};