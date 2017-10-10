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
    ListNode* firstLess   = 0;
    ListNode* lastLess    = 0;
    ListNode* lastBigger  = 0;
    
    ListNode* partition(ListNode* head, int x) {
        if(head == 0) return 0;
                
        if(head -> val < x){
            lastLess = head;
            firstLess = lastLess;
        }
        else{
            lastBigger = head;   
        }
        
        partition(head -> next, x);
                
        if(head -> val < x){
            if(lastLess != head){
                head -> next = lastLess;
                lastLess = head;
            }                
        }
        else{
            if(lastBigger != head){
                head -> next = lastBigger;
                lastBigger = head;
            }        
            else{
                lastBigger -> next = 0;
            }
        }
        
        if(firstLess != 0)
            firstLess -> next = lastBigger;
        
        if(lastLess == 0) return lastBigger;
        else return lastLess;     
    }
};
