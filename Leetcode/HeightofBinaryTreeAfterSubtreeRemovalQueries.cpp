// https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries

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
    int build(TreeNode* root,
              vector<set<pair<int, int>>>& deps,
              map<int, int>& vd,
              int dep) {
        if (!root) {
            return 0;
        }

        if (deps.size() == dep) {
            deps.push_back({});
        }

        vd[root->val] = dep;

        auto left = build(root->left, deps, vd, dep + 1);
        auto right = build(root->right, deps, vd, dep + 1);

        deps[dep].insert({
            1 + max(left, right),
            root->val,
        });

        return 1 + max(left, right);
    }

    vector<int> treeQueries(TreeNode* root, vector<int>& queries) {
        vector<set<pair<int, int>>> deps;
        map<int, int> vd;
        build(root, deps, vd, 0);

        vector<int> ans;
        for (const auto q : queries) {
            const auto d = vd[q];
            auto last = --deps[d].end();
            auto to_add = last->first;
            if (last->second == q) {
                if (last == deps[d].begin()) {
                    to_add = 0;
                } else {
                    --last;
                    to_add = last->first;
                }
            }
            ans.push_back(d + to_add - 1);
        }

        return ans;
    }
};