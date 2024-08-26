// https://leetcode.com/problems/n-ary-tree-postorder-traversal

/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
   public:
    vector<int> postorder(Node* root) {
        vector<int> ans;
        if (root) {
            for (auto& child : root->children) {
                auto child_ans = postorder(child);
                ans.insert(ans.end(),
                           std::make_move_iterator(child_ans.begin()),
                           std::make_move_iterator(child_ans.end()));
            }
            ans.push_back(root->val);
        }

        return ans;
    }
};