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

    int maxLevel = -1;
    vector<int> ans;
    
    void dfs(TreeNode* node, int curLevel){
        if(curLevel > maxLevel){
           ans.push_back(node -> val); 
           maxLevel = curLevel;
        }
        if(node->right != NULL) dfs(node -> right, curLevel + 1);
        if(node->left  != NULL) dfs(node -> left , curLevel + 1);
        
    }

    vector<int> rightSideView(TreeNode* root) {
        if(root == NULL) return ans;
        dfs(root, 0);
        return ans;
    }
};
