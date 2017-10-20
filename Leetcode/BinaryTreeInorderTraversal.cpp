/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    map<TreeNode*, int> used;
    vector<int> inorderTraversal(TreeNode* root) {
        stack<TreeNode*>s;
        vector<int> ans;
        if(root == NULL) return ans;
        s.push(root);
        while(!s.empty()){
            TreeNode* t = s.top();
            if(t->left != NULL && !used[t->left]) s.push(t->left);
            else {
                ans.push_back(t->val);
                used[t] = 1;
                s.pop();
                if(t->right != NULL && !used[t->right]) s.push(t->right);
            }
        }
        return ans;
    }
};
