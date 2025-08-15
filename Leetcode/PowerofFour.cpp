// https://leetcode.com/problems/power-of-four

class Solution {
   public:
    bool isPowerOfFour(int n) {
        using i64 = long long;

        if (n <= 0) {
            return false;
        }

        i64 cur = 1;
        while (cur <= n) {
            if (cur == n) {
                return true;
            }
            cur *= 4;
        }

        return false;
    }
};