// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree

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
   public:
    void solve(TreeNode* root, int level, vector<long long>& levels) {
        if (!root) {
            return;
        }
        if (levels.size() == level) {
            levels.push_back(0);
        }
        levels[level] += root->val;
        solve(root->left, level + 1, levels);
        solve(root->right, level + 1, levels);
    }

    long long kthLargestLevelSum(TreeNode* root, int k) {
        vector<long long> levels;
        solve(root, 0, levels);
        sort(levels.begin(), levels.end(), std::greater<long long>());

        if (levels.size() > k - 1) {
            return levels[k - 1];
        } else {
            return -1;
        }
    }
};