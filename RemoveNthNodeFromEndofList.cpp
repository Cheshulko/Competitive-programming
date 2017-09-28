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
	int cntFromEnd = 0;
	ListNode* removeNthFromEnd(ListNode* head, int n) {
		ListNode* res;
		if (head != NULL){
			head->next = removeNthFromEnd(head->next, n);
			++cntFromEnd;
			if (cntFromEnd == n){
				res = head->next;
			}
			else{
				res = head;
			}
		}
		else{
			res = head;
		}
		return res;
	}
};