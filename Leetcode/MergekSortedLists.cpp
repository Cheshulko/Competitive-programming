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
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        if(lists.size() == 0) return nullptr;
        
        ListNode* start = nullptr;
        ListNode* tail = nullptr;
        ListNode* curMin = nullptr;
        int cntDone = 0;
        
        while(cntDone != lists.size()){
            curMin = nullptr;
            cntDone = 0;
            ListNode** linkToMin = nullptr;
            for(auto& node: lists){
                if(node == nullptr){
                    ++cntDone; 
                }else if(curMin && curMin->val > node->val || !curMin){
                    linkToMin = &node;
                    curMin = node;
                }
            }
            if(!start){
                start = curMin;
                tail = start;
            }else{
                tail->next = curMin;
                tail = tail->next;
            }
            if(linkToMin != nullptr)
                (*linkToMin) = (*linkToMin)->next;
        }
        return start;
    }
};