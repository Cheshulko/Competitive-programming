// https://leetcode.com/problems/find-numbers-with-even-number-of-digits

int findNumbers(int* nums, int numsSize) {
    int odd = 0;
    for (int i = 0; i < numsSize; ++i) {
        odd += (int)(1 + log10(nums[i])) & 1;
    }

    return numsSize - odd;
}