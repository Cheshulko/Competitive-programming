// https://leetcode.com/problems/linked-list-in-binary-tree

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
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

struct Solution {
}

class Solution {
   public:
    bool isSubPath(ListNode* head, TreeNode* root) {
        if (root == nullptr) {
            return head == nullptr;
        }

        if (solve(head, root)) {
            return true;
        } else {
            return isSubPath(head, root->left) || isSubPath(head, root->right);
        }
    }

   private:
    bool solve(ListNode* head, TreeNode* cur) {
        if (head == nullptr) {
            return true;
        }
        if (cur == nullptr) {
            return false;
        }
        if (cur->val != head->val) {
            return false;
        }

        return solve(head->next, cur->left) || solve(head->next, cur->right);
    }
};