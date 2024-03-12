// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list

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
    ListNode* removeZeroSumSublists(ListNode* head) {
        if (!head) {
            return head;
        }

        auto sum = 0;
        auto next = head;

        while (next) {
            sum += next->val;

            if (sum == 0) {
                return Solution::removeZeroSumSublists(next->next);
            } else {
                next = next->next;
            }
        }

        head->next = Solution::removeZeroSumSublists(head->next);
        return head;
    }
};