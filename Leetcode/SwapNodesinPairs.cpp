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
    ListNode* swapPairs(ListNode* head) {
        if(head == NULL) return NULL;
        if(head -> next == NULL) return head;
        ListNode* t = head -> next -> next;
        ListNode* fir = head;
        ListNode* sec = head -> next;
        {
            sec -> next = fir;
            fir -> next = swapPairs(t);
            return sec;
        }
    }
};
