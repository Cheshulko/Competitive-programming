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

    TreeNode *build(vector<int> &nums, int l, int r){
        if(l > r) return NULL;
        int m = ((r - l) / 2) + ((r - l) % 2) + l; 
        TreeNode *n = new TreeNode(nums[m]);
        n->left = build(nums, l, m-1);
        n->right = build(nums, m + 1, r);
        return n;
    }

    TreeNode* sortedArrayToBST(vector<int>& nums) {
        if(nums.size() == 0) return NULL;
        return build(nums, 0, nums.size() - 1);
    }
};
