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
    vector< vector<int> > levelOrderBottom(TreeNode* root) {
        
        vector< vector<int> > ans;
        if(root == NULL) return ans;
        
        queue< TreeNode* > q;
        int level = 0;
        q.push(root);
        int onTheLevel = 1;
        int nextLevel;
        
        while(!q.empty()){
            ans.push_back(vector<int>());
            nextLevel = 0;
            for(int i = 0; i < onTheLevel; ++i){
                TreeNode *cur = q.front();
                q.pop();
                ans[level].push_back(cur->val);
                if(cur->left != NULL){
                    nextLevel++;
                    q.push(cur->left);
                }
                if(cur->right != NULL){
                    nextLevel++;
                    q.push(cur->right);
                }
            }
            level++;
            onTheLevel = nextLevel;
        }
        
        reverse(ans.begin(), ans.end());
        return ans;
    }
};

