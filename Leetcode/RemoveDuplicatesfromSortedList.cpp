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
    ListNode* deleteDuplicates(ListNode* head) {
        if(head == NULL) return head;
        ListNode *cur = head, *tmp;
        while(cur != NULL){
            tmp = cur -> next;
            while(tmp != NULL && cur -> val == tmp -> val){
                ListNode *d = tmp;
                tmp = tmp -> next;
                delete d;
            }
            cur -> next = tmp;
            cur = tmp;
        }
        return head;
    }
};
