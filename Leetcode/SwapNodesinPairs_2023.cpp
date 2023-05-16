// https://leetcode.com/problems/swap-nodes-in-pairs

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
    ListNode* swapPairs(ListNode*& head, bool odd = true) {
        ListNode* next;
        if (head != nullptr) {
            next = swapPairs(head->next, !odd);

            if (odd) {
                auto*& cur = head;
                auto*& next = head->next;

                if (next) {
                    auto* next_to_cur = head->next;
                    auto* next_to_next = next ? next->next : nullptr;

                    std::swap(*cur, *next);

                    cur->next = next_to_cur;
                    next->next = next_to_next;
                }

                return cur;
            }
        }

        return head;
    }
};