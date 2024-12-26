// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop

class Solution {
   public:
    vector<int> finalPrices(vector<int>& prices) {
        auto compare = [](const int* const a, const int* const b) {
            return *a < *b;
        };

        priority_queue<int*, vector<int*>, decltype(compare)> max_queue;

        for (auto& price : prices) {
            while (!max_queue.empty() && *max_queue.top() >= price) {
                *max_queue.top() -= price;
                max_queue.pop();
            }

            max_queue.push(&price);
        }

        return prices;
    }
};