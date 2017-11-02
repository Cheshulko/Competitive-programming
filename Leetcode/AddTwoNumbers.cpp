You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2, int up = 0) {
        if(l1 == NULL){
            if(l2 == NULL){
                if(up != 0){
                    ListNode *el = new ListNode(up);
                    return el;
                }
                else{
                    return NULL;
                }
            }
            else{
                int x = l2 -> val + up;
                up = x / 10;
                x %= 10;
                ListNode *el = new ListNode(x);
                el -> next = addTwoNumbers(NULL, l2 -> next, up);
                return el;
            }
        }
        else{
            if(l2 == NULL){
                int x = l1 -> val + up;
                up = x / 10;
                x %= 10;
                ListNode *el = new ListNode(x);
                el -> next = addTwoNumbers(l1 -> next, NULL, up);
                return el;
            }
            else{
                int x = l1 -> val + l2 -> val + up;
                up = x / 10;
                x %= 10;
                ListNode *el = new ListNode(x);
                el -> next = addTwoNumbers(l1 -> next, l2 -> next, up);
                return el;
            }
        }
    }
};
