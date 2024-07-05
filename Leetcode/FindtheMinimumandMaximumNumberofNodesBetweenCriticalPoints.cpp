// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points

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

const int inf = 1e9;

class Solution {
   public:
    vector<int> nodesBetweenCriticalPoints(ListNode* head) {
        int mn = inf, mx = -inf;
        go(mn, mx, head, 0, 0, 1);

        if (mn == inf) {
            return {-1, -1};
        }

        return {mn, mx};
    }

   private:
    void go(int& mn, int& mx, ListNode* head, int f, int cur, int d) {
        if (!head) {
            return;
        }
        auto next1 = head->next;
        if (!next1) {
            return;
        }
        auto next2 = next1->next;
        if (!next2) {
            return;
        }

        if ((head->val < next1->val && next1->val > next2->val) ||
            (head->val > next1->val && next1->val < next2->val)) {
            if (f == 0) {
                f = d;
            } else {
                mn = min(mn, d - cur);
            }

            cur = d;
            mx = max(mx, cur - f);
        }

        go(mn, mx, head->next, f, cur, d + 1);
    }
};