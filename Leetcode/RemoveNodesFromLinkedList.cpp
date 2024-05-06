// https://leetcode.com/problems/remove-nodes-from-linked-list

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
    ListNode* removeNodes(ListNode* head) {
        if (!head) {
            return head;
        }

        auto next = removeNodes(head->next);
        head->next = next;

        if (!next) {
            return head;
        } else {
            if (next->val > head->val) {
                return next;
            } else {
                return head;
            }
        }
    }
};