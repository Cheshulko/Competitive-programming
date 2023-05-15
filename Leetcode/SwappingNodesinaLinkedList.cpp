// https://leetcode.com/problems/swapping-nodes-in-a-linked-list

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
    ListNode* swapNodes(ListNode* head, int k) {
        int cnt = 0;
        ListNode* lhead = head;
        ListNode* kth_from_start = nullptr;
        ListNode* kth_from_end = nullptr;
        int cur_end = 1;

        while (lhead != nullptr) {
            cnt++;
            lhead = lhead->next;
        }
        k = std::min(k, cnt - k + 1);

        return go(head, k, 1, cur_end, kth_from_start, kth_from_end);
    }

    ListNode* go(ListNode* head,
                 int k,
                 int cur,
                 int& cur_end,
                 ListNode*& kth_from_start,
                 ListNode*& kth_from_end) {
        if (head != nullptr) {
            if (k == cur) {
                kth_from_start = head;
            }

            auto next = go(head->next, k, cur + 1, cur_end, kth_from_start,
                           kth_from_end);

            if (k == cur_end) {
                kth_from_end = head;
            }

            if (k == cur) {
                head = kth_from_end;
            } else if (k == cur_end) {
                head = kth_from_start;
            }

            cur_end++;

            head->next = next;

            return head;
        }
        return nullptr;
    }
};