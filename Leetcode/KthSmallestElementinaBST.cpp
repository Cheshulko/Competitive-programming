/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    int cnt(TreeNode* root){
        if(root != 0) return 1 + cnt(root -> left) + cnt(root -> right);
        else return 0;
    }
    int kthSmallest(TreeNode* root, int k) {
         
         int x = cnt(root -> left);
         if(x + 1 == k) return root -> val;
         else if(k < x + 1) return kthSmallest(root->left, k);
         else return kthSmallest(root->right, k - x - 1);
    }
};
