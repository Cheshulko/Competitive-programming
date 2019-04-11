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
    ListNode* reverseList(ListNode* head) {
        if(head == NULL) return head;
        if(head->next == NULL) return head;
        
        ListNode *newHead = reverseList(head->next);
        head->next->next = head;
        head->next = NULL;
        return newHead;
        
    }
};

class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        if(head == NULL) return head;
        ListNode* f = head;
        ListNode* s = head->next;
        ListNode* tmp;
        
        f->next = NULL;
        
        while(s != NULL){
            tmp = s->next;
            s->next = f;
            f = s;
            s = tmp;
        }
        
        return f;
    }
};
