// https://leetcode.com/problems/distribute-coins-in-binary-tree

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
    int ans = 0;

    int go(TreeNode* root) {
        if (!root) {
            return 0;
        }

        auto left = go(root->left);
        auto right = go(root->right);

        auto cur = root->val;
        cur += left;
        cur += right;
        cur -= 1;

        ans += abs(cur);

        return cur;
    }

    int distributeCoins(TreeNode* root) {
        go(root);

        return ans;
    }
};