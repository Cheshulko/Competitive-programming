// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree

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
    void dfs(TreeNode* node, size_t dep, vector<long long>& level) {
        if (!node) {
            return;
        }

        if (level.size() == dep) {
            level.push_back(0);
        }

        level[dep] += node->val;
        dfs(node->left, dep + 1, level);
        dfs(node->right, dep + 1, level);
    }

    int maxLevelSum(TreeNode* root) {
        vector<long long> level;
        dfs(root, 0, level);

        long long ma = numeric_limits<long long>::min();
        size_t ind = 0;

        for (size_t i = 0; i < level.size(); ++i) {
            if (level[i] > ma) {
                ma = level[i];
                ind = i;
            }
        }

        return ind + 1;
    }
};