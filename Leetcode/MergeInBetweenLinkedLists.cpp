// https://leetcode.com/problems/merge-in-between-linked-lists

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
    ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
        if (!list1) {
            return nullptr;
        }

        if (a > 1) {
            list1->next =
                Solution::mergeInBetween(list1->next, a - 1, b - 1, list2);
            return list1;
        } else {
            auto next_from_list1 = list1->next;

            while (b > 0 && next_from_list1 && next_from_list1->next) {
                next_from_list1 = next_from_list1->next;
                b -= 1;
            }

            list1->next = list2;

            while (list2 && list2->next) {
                list2 = list2->next;
            }

            list2->next = next_from_list1;
            return list1;
        }
    }
};