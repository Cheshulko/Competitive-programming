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
    vector<vector<int>> levelOrder(TreeNode* root) {
        queue<TreeNode*> q;
        vector< vector<int> > ans;
        if(root == NULL) return ans;
        q.push(root);
        int level = 0;
        int curOnLevel = 1;
        int nextLevel;
        while(!q.empty()){
            ans.push_back(vector<int>());
            nextLevel = 0;
            for(int i = 0; i < curOnLevel; ++i){
                TreeNode* cur = q.front();
                q.pop();
                ans[level].push_back(cur->val);
                if(cur -> left != NULL) q.push(cur->left), nextLevel++;
                if(cur -> right != NULL) q.push(cur->right), nextLevel++;
            }
            curOnLevel = nextLevel;
            nextLevel = 0;
            level++;
        }
        return ans;
    }
};
