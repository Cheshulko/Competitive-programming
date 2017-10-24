/**
 * Definition for binary tree with next pointer.
 * struct TreeLinkNode {
 *  int val;
 *  TreeLinkNode *left, *right, *next;
 *  TreeLinkNode(int x) : val(x), left(NULL), right(NULL), next(NULL) {}
 * };
 */
class Solution {
public:
    void connect(TreeLinkNode *root) {
        if(root == NULL) return;
        queue<pair<TreeLinkNode*, int> > q;
        q.push(pair<TreeLinkNode*, int>(root, 0));
        
        while(!q.empty()){
            pair<TreeLinkNode*, int> x = q.front();
            q.pop();
            if(!q.empty()){
                if(q.front().second == x.second) x.first->next = q.front().first;
                else x.first->next = NULL;
            }
            else{
                x.first->next = NULL;
            }
            if(x.first->left != NULL) q.push(pair<TreeLinkNode*, int>(x.first->left, x.second + 1));
            if(x.first->right != NULL) q.push(pair<TreeLinkNode*, int>(x.first->right, x.second + 1));
        }
    }
};
