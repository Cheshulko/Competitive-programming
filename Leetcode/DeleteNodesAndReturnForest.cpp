// https://leetcode.com/problems/delete-nodes-and-return-forest

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
    bool go(TreeNode* root,
            bool is_root,
            const set<int>& to_delete,
            vector<TreeNode*>& res) {
        if (!root) {
            return false;
        }

        auto need_delete = to_delete.cend() != to_delete.find(root->val);

        if (is_root && !need_delete) {
            res.push_back(root);
        }

        if (go(root->left, need_delete, to_delete, res)) {
            root->left = nullptr;
        };
        if (go(root->right, need_delete, to_delete, res)) {
            root->right = nullptr;
        }

        return need_delete;
    }

    vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
        set<int> s;
        for (auto x : to_delete) {
            s.insert(x);
        }
        vector<TreeNode*> ans;
        go(root, true, s, ans);

        return ans;
    }
};