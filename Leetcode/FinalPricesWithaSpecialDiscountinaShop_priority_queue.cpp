// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop

class Solution {
   public:
    vector<int> finalPrices(vector<int>& prices) {
        const auto n = prices.size();

        vector<int> ans = prices;
        priority_queue<pair<int, int>> max_priority_queue;

        for (auto i = 0; i < n; ++i) {
            const auto price = prices[i];

            while (!max_priority_queue.empty() &&
                   max_priority_queue.top().first >= price) {
                ans[max_priority_queue.top().second] -= price;
                max_priority_queue.pop();
            }

            max_priority_queue.push({price, i});
        }

        return ans;
    }
};