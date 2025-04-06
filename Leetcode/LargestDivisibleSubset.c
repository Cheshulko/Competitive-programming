// https://leetcode.com/problems/largest-divisible-subset

#define N 1000

int comp(const void* a, const void* b) {
    int* ai = (int*)a;
    int* bi = (int*)b;

    return *ai - *bi;
}

int* largestDivisibleSubset(int* nums, int numsSize, int* returnSize) {
    qsort(nums, numsSize, sizeof(int), comp);

    int dp[N];
    int par[N];

    for (int i = 0; i < numsSize; ++i) {
        dp[i] = 1;
        par[i] = i;
    }

    int ma_i = 0;
    for (int i = 0; i < numsSize; ++i) {
        for (int j = i + 1; j < numsSize; ++j) {
            if (nums[j] % nums[i] == 0 && dp[j] < dp[i] + 1) {
                dp[j] = dp[i] + 1;
                par[j] = i;
            }

            if (dp[ma_i] < dp[j]) {
                ma_i = j;
            }
        }
    }

    *returnSize = dp[ma_i];
    int* ans = (int*)malloc(sizeof(int) * dp[ma_i]);
    while (true) {
        ans[dp[ma_i] - 1] = nums[ma_i];

        if (ma_i == par[ma_i]) {
            break;
        }
        ma_i = par[ma_i];
    }

    return ans;
}