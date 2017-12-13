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
    ListNode *detectCycle(ListNode *head) {
        ListNode *p1 = head, *p2 = head;
        bool have = false;
        while(p1 && p2 && p2 -> next){
            p1 = p1 -> next;
            p2 = p2 -> next -> next;
            if(p1 == p2){
                have = true;
                break;
            } 
        }
        if(!have) return NULL;
        
        ListNode *p = p1;
        int len = 0;
        while(p -> next != p2) ++len, p = p -> next;
        ++len;
        p1 = head, p2 = head;
        for(int i = 0; i < len; ++i) p2 = p2 -> next;
        while(p1 != p2){
            p1 = p1 -> next;
            p2 = p2 -> next;
        }
        return p1;
    }
};
