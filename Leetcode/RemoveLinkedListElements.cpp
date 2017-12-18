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
    ListNode* removeElements(ListNode* head, int val) {
        while(head != NULL && head -> val == val){
            ListNode *ext = head;
            head = head -> next;
            delete ext;
        }
        if(head == NULL) return head;
        
        head->next = removeElements(head -> next, val);
        
        return head;
        
    }
};
