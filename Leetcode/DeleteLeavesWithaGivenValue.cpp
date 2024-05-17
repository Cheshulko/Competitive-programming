// https://leetcode.com/problems/delete-leaves-with-a-given-value

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
    TreeNode* removeLeafNodes(TreeNode* root, int target) {
        if (!root) {
            return root;
        }

        auto left = Solution::removeLeafNodes(root->left, target);
        auto right = Solution::removeLeafNodes(root->right, target);

        root->left = left;
        root->right = right;

        if (!left && !right && root->val == target) {
            return nullptr;
        } else {
            return root;
        }
    }
};