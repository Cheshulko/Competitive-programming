// https://leetcode.com/problems/minimum-penalty-for-a-shop

class Solution {
   public:
    int bestClosingTime(string customers) {
        const auto n = customers.size();

        int penalty_closed = 0;
        for (int i = 0; i < n; ++i) {
            penalty_closed += customers[i] == 'Y';
        }

        int penalty = 0;
        int best = numeric_limits<int>::max();
        int j = 0;
        for (int i = 0; i <= n; ++i) {
            int maybe_best = penalty_closed + penalty;
            if (maybe_best < best) {
                best = maybe_best;
                j = i;
            }

            if (i < n) {
                penalty += customers[i] == 'N';
                penalty_closed -= customers[i] == 'Y';
            }
        }

        return j;
    }
};