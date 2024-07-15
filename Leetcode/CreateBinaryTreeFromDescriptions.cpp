// https://leetcode.com/problems/create-binary-tree-from-descriptions

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
    TreeNode* createBinaryTree(vector<vector<int>>& descriptions) {
        map<int, int> left;
        map<int, int> right;
        set<int> nodes;
        set<int> children;

        for (const auto& item : descriptions) {
            children.insert(item[1]);
            nodes.insert(item[0]);
            nodes.insert(item[1]);

            if (item[2]) {
                left[item[0]] = item[1];
            } else {
                right[item[0]] = item[1];
            }
        }

        auto root = -1;
        for (const auto& node : nodes) {
            if (children.cend() == children.find(node)) {
                root = node;
                break;
            }
        }

        auto rootNode = new TreeNode(root);

        vector<TreeNode*> q;
        q.push_back(rootNode);

        while (q.size()) {
            auto cur = q.back();
            q.pop_back();

            if (auto leftChild = left.find(cur->val);
                left.cend() != leftChild) {
                auto leftNode = new TreeNode(leftChild->second);
                cur->left = leftNode;
                q.push_back(leftNode);
            }

            if (auto rightChild = right.find(cur->val);
                right.cend() != rightChild) {
                auto rightNode = new TreeNode(rightChild->second);
                cur->right = rightNode;
                q.push_back(rightNode);
            }
        }

        return rootNode;
    }
};