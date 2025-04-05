// https://leetcode.com/problems/sum-of-all-subset-xor-totals

int subsetXORSum(int* nums, int numsSize) {
    int ans = 0;

    for (int mask = 0; mask < (1 << numsSize); ++mask) {
        int cur = 0;
        for (int i = 0; i < numsSize; ++i) {
            cur ^= ((mask & (1 << i)) > 0) * nums[i];
        }
        ans += cur;
    }

    return ans;
}