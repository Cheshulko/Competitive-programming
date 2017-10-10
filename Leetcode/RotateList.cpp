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
    ListNode* root =  0;
    ListNode* last =  0;
    int realK      = -1;
    int SZ         = -1;
    
    ListNode* rotateRight(ListNode* head, int k, int sz = 1) {     
        if(sz == 1) root = head;
        if(k == 0) return head;
        if(head == 0){
            if(sz > 1){
                SZ = sz - 1;
                realK = k % SZ;                
            }
            return 0;
        }      
        
        if(head -> next == 0){
            last = head;
        }
        
        ListNode* st = rotateRight(head -> next, k, sz + 1);
        if(st == 0) st = root;
        
        if(realK && SZ - sz == realK){
            ListNode* tmp = head -> next; 
            head -> next = 0;
            last -> next = root;
            return tmp;
        }       
        
        return st;        
    }
};
