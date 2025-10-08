// https://leetcode.com/problems/successful-pairs-of-spells-and-potions

class Solution {
   public:
    vector<int> successfulPairs(vector<int>& spells,
                                vector<int>& potions,
                                long long success) {
        sort(potions.begin(), potions.end());

        vector<int> ans;
        transform(
            spells.begin(), spells.end(), back_inserter(ans), [&](int spell) {
                auto mi = (success + spell - 1) / spell - 1;
                auto l = upper_bound(potions.cbegin(), potions.cend(), mi);

                return potions.cend() - l;
            });

        return ans;
    }
};