// https://leetcode.com/problems/construct-the-minimum-bitwise-array-i

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* minBitwiseArray(int* nums, int numsSize, int* returnSize) {
    *returnSize = numsSize;

    int* ans = (int*)malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; ++i) {
        ans[i] = -1;

        for (int x = 0; x <= nums[i]; ++x) {
            if ((x | (x + 1)) == nums[i]) {
                ans[i] = x;

                break;
            }
        }
    }

    return ans;
}