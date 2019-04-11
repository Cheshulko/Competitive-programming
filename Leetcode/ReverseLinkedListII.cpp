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
    ListNode* reverseBetween(ListNode* head, int m, int n) {
        if(head == NULL) return head;
        
        ListNode *st = head;
        
        ListNode *f = NULL, *s = NULL, *tmp = NULL;
        
        ListNode *lastNoReversed = NULL, *firstInTail = NULL;
        
        while(m > 1){
            lastNoReversed = head;
            head = head->next;
            --m, --n;
        }
        
        firstInTail = head;
        f = head;
        s = head->next;
        
        while(n > 1){
            tmp = s->next;
            s->next = f;
            f = s;
            s = tmp;
            
            --n;
        }
        
        if(lastNoReversed) lastNoReversed->next = f;
        firstInTail->next = s;
        
        return (lastNoReversed ? st : f);
    }
};