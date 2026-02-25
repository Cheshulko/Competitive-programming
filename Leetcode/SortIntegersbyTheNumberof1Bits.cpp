// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits

class Solution {
   public:
    vector<int> sortByBits(vector<int>& arr) {
        sort(arr.begin(), arr.end(), [](const auto a, const auto b) {
            // return popcount(a) < popcount(b);

            int ac = 0;
            for (auto aa = a; aa; ac += (aa & 1), aa >>= 1) {
            }

            int bc = 0;
            for (auto bb = b; bb; bc += (bb & 1), bb >>= 1) {
            }

            return (ac < bc) || (ac == bc && a < b);
        });

        return arr;
    }
};