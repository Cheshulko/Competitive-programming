// https://leetcode.com/problems/binary-tree-right-side-view

// struct TreeNode {
//     int val;
//     TreeNode* left;
//     TreeNode* right;
//     TreeNode() : val(0), left(nullptr), right(nullptr) {}
//     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
//     TreeNode(int x, TreeNode* left, TreeNode* right)
//         : val(x), left(left), right(right) {}
// };

class Solution {
   public:
    vector<int> rightSideView(TreeNode* root) {
        vector<int> ans;
        dfs(root, 0, ans);
        return ans;
    }

   private:
    void dfs(TreeNode* cur, int deps, vector<int>& ans) {
        if (!cur) {
            return;
        }

        if (ans.size() <= deps) {
            ans.push_back(cur->val);
        }

        dfs(cur->right, deps + 1, ans);
        dfs(cur->left, deps + 1, ans);
    }
};