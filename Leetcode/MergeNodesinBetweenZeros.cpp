// https://leetcode.com/problems/merge-nodes-in-between-zeros

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
    ListNode* mergeNodes(ListNode* head) {
        if (!head) {
            return nullptr;
        }

        if (head->val == 0) {
            if (head->next == nullptr) {
                return nullptr;
            }

            auto cur = head->next;
            while (cur && cur->val != 0) {
                head->val += cur->val;
                cur = cur->next;
            }
            head->next = mergeNodes(cur);

            return head;
        } else {
            return nullptr;
        }
    }
};