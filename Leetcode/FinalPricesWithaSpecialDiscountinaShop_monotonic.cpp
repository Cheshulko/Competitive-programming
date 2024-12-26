// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop

class Solution {
   public:
    vector<int> finalPrices(vector<int>& prices) {
        reverse(prices.begin(), prices.end());

        vector<int> ans;
        vector<int> monotonic;
        for (const auto price : prices) {
            while (!monotonic.empty() && monotonic.back() > price) {
                monotonic.pop_back();
            }

            ans.push_back(monotonic.empty() ? price : price - monotonic.back());
            monotonic.push_back(price);
        }
        reverse(ans.begin(), ans.end());

        return ans;
    }
};