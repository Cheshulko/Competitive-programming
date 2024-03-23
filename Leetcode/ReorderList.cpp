// https://leetcode.com/problems/reorder-list

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
    void reorderList(ListNode* head) {
        auto* slow = head;
        auto* fast = head->next;

        while (fast && fast->next) {
            fast = fast->next->next;
            slow = slow->next;
        }

        std::stack<ListNode*> tail;
        slow = slow->next;
        // slow -> at the middle

        // reversed second half of the list
        while (slow) {
            tail.push(slow);
            slow = slow->next;
        }

        auto first_half_head = head;
        while (!tail.empty()) {
            auto* last = tail.top();
            tail.pop();

            first_half_head = first_half_head->next;
            head->next = last;
            last->next = first_half_head;

            head = first_half_head;
        }

        first_half_head->next = nullptr;
    }
};