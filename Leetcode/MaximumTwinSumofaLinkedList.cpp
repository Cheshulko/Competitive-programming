// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list

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
    int pairSum(ListNode* head) {
        queue<int> q;
        return go(head, q);
    }

    int go(ListNode* head, queue<int>& q) {
        if (head != nullptr) {
            q.push(head->val);
            const auto max_prev = go(head->next, q);
            const auto paired = q.front();
            q.pop();

            return std::max(max_prev, paired + head->val);
        }
        return 0;
    }
};