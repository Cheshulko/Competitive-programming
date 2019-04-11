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
    int numComponents(ListNode* head, vector<int>& G) {
        if(head == NULL) return 0;
        
        sort(G.begin(), G.end());
        
        int index = -1;
        int ans = 0;
        int con = 0;
        while(head != NULL){
            index = upper_bound(G.begin(), G.end(), head->val) - G.begin();
            --index;
            if(index >= 0 && G[index] != head->val || index < 0){
                ans += con;
                con = 0;
            }else if(index >= 0 && G[index] == head->val){
                con = 1;
            }
            head = head->next;
        }
        
        return ans + con;
    }
};