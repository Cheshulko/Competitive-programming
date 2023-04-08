// https://leetcode.com/problems/clone-graph

#include <cmath>
#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;

class Node {
   public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};

class Solution {
    vector<pair<bool, Node*>> done;

   public:
    Solution() { done.resize(102); }
    Node* cloneGraph(Node* node) {
        if (!node) {
            return nullptr;
        }
        int val = node->val;
        vector<Node*> neighborsCopied;
        neighborsCopied.resize(node->neighbors.size());

        auto copied = new Node(val, std::move(neighborsCopied));
        done[val].second = copied;

        done[val].first = true;

        for (int i = 0; i < node->neighbors.size(); ++i) {
            auto to = node->neighbors[i];
            auto to_val = to->val;
            if (!done[to_val].first) {
                cloneGraph(to);
            }
        }

        for (int i = 0; i < node->neighbors.size(); ++i) {
            copied->neighbors[i] = done[node->neighbors[i]->val].second;
        }
        return copied;
    }
};