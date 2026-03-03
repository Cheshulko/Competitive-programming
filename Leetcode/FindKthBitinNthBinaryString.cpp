// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string

class Solution {
   public:
    char findKthBit(int n, int k, int rev = 0) {
        const auto len = (1 << n) - 1;

        if (len == 1) {
            assert(k == 1);
            return rev + '0';
        } else if (k - 1 == len / 2) {
            return (rev ^ 1) + '0';
        } else if (k - 1 < len / 2) {
            return findKthBit(n - 1, k, rev);
        } else {
            return findKthBit(n - 1, len - k + 1, rev ^ 1);
        }
    }
};