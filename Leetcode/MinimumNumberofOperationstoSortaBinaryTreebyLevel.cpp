// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level

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
    int minSwaps(const vector<int>& arr) {
        auto n = arr.size();
        pair<int, int> arrPos[n];
        for (int i = 0; i < n; i++) {
            arrPos[i].first = arr[i];
            arrPos[i].second = i;
        }
        sort(arrPos, arrPos + n);

        vector<bool> vis(n, false);

        int ans = 0;
        for (int i = 0; i < n; i++) {
            if (vis[i] || arrPos[i].second == i) {
                continue;
            }
            int cycle_size = 0;
            int j = i;
            while (!vis[j]) {
                vis[j] = 1;
                j = arrPos[j].second;
                cycle_size++;
            }
            if (cycle_size > 0) {
                ans += (cycle_size - 1);
            }
        }

        return ans;
    }

    void dfs(TreeNode* root, int level, vector<vector<int>>& levels) {
        if (!root) {
            return;
        }
        if (levels.size() == level) {
            levels.push_back({});
        }
        levels[level].push_back(root->val);
        dfs(root->left, level + 1, levels);
        dfs(root->right, level + 1, levels);
    }

    int minimumOperations(TreeNode* root) {
        vector<vector<int>> levels;
        dfs(root, 0, levels);

        auto ans = 0;
        for (const auto& level : levels) {
            ans += minSwaps(level);
        }

        return ans;
    }
};