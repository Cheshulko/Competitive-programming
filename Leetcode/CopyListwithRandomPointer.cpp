// https://leetcode.com/problems/copy-list-with-random-pointer

/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
   public:
    Node* copyRandomList(Node* head) {
        if (!head) {
            return nullptr;
        }

        auto head_new = new Node(head->val);

        auto cur = head;
        auto cur_new = head_new;

        while (cur && cur->next) {
            cur = cur->next;
            auto next_new = new Node(cur->val);
            cur_new->next = next_new;
            cur_new = next_new;
        }

        cur = head;
        cur_new = head_new;

        while (cur) {
            auto cur_to = cur->random;

            if (cur_to) {
                auto cur_local = head;
                auto cur_new_local = head_new;

                while (cur_local != cur_to) {
                    cur_local = cur_local->next;
                    cur_new_local = cur_new_local->next;
                }

                cur_new->random = cur_new_local;
            }

            cur = cur->next;
            cur_new = cur_new->next;
        }

        return head_new;
    }
};