// https://leetcode.com/problems/binary-tree-postorder-traversal

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left),
 * right(right) {}
 * };
 */
class Solution {
   public:
    vector<int> postorderTraversal(TreeNode* root) {
        vector<int> ans;
        if (root) {
            auto left = postorderTraversal(root->left);
            auto right = postorderTraversal(root->right);
            ans.insert(ans.end(), std::make_move_iterator(left.begin()),
                       std::make_move_iterator(left.end()));
            ans.insert(ans.end(), std::make_move_iterator(right.begin()),
                       std::make_move_iterator(right.end()));
            ans.push_back(root->val);
        }

        return ans;
    }
};