// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array

int countPairs(int* nums, int numsSize, int k) {
    int ans = 0;
    for (int i = 0; i < numsSize; ++i) {
        for (int j = i + 1; j < numsSize; ++j) {
            ans += ((nums[i] == nums[j]) && (i * j % k == 0));
        }
    }

    return ans;
}