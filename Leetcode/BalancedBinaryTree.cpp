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
    int dfs(TreeNode* r, bool &OK){
        if(r == NULL) return 0;
        int le = dfs(r->left, OK);
        int ri = dfs(r->right, OK);
        if(abs(ri - le) > 1) OK = 0;
        return max(ri, le) + 1;
    }
    bool isBalanced(TreeNode* root) {
        //if(root == NULL) return true;
        bool OK = 1;
        dfs(root, OK);
        return OK;
    }
};
