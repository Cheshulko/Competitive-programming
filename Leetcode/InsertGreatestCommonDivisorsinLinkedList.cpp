// https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list

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
    int gcd(int a, int b) {
        while (b != 0) {
            a %= b;
            swap(a, b);
        }
        return a;
    }
    ListNode* insertGreatestCommonDivisors(ListNode* head) {
        if (head && head->next) {
            auto node = new ListNode(gcd(head->val, head->next->val));
            auto next = head->next;
            head->next = node;
            node->next = next;
            insertGreatestCommonDivisors(next);
        }
        return head;
    }
};