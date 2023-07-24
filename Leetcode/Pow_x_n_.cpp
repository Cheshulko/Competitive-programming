// https://leetcode.com/problems/powx-n

class Solution {
   public:
    double myPow(double x, long long n) {
        if (n < 0) {
            return 1.0 / Solution::myPow(x, -n);
        } else if (n == 0) {
            return 1.0;
        } else {
            const auto y = Solution::myPow(x, n / 2);
            if (n % 2 == 1) {
                return y * y * x;
            } else {
                return y * y;
            }
        }
    }
};