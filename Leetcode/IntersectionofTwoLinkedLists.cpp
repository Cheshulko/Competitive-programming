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
    map<ListNode*, bool> used; 
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode *cur = headA;
        while(cur != NULL){
            used[cur] = 1;
            cur = cur -> next;
        }
        cur = headB;
        while(cur != NULL){
            if(used[cur]) return cur;
            cur = cur -> next;
        }
        return NULL;
    }
};
