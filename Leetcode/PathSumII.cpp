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
    vector<vector<int>> ans;
    void dfs(TreeNode *cur, vector<int> &v, int &s, int &sum){
        v.push_back(cur -> val);
        s += cur -> val;
        if(cur -> left == NULL && cur -> right == NULL){
            if(s == sum) ans.push_back(v);
        }
        else {
            if(cur -> left != NULL)dfs(cur -> left, v, s, sum);
             if(cur -> right != NULL)dfs(cur -> right, v, s, sum);
        }
        s -= cur->val;
        v.pop_back();
    }

    vector<vector<int>> pathSum(TreeNode* root, int sum) {
        if(root == NULL) return ans;
        int s = 0;
        vector<int> v;
        dfs(root, v, s, sum);
        return ans;
    }
};
