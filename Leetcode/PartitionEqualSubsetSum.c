// https://leetcode.com/problems/partition-equal-subset-sum

#define MAX_DIF 100 * 200

int dp_[2 * MAX_DIF + 1] = {0};
int ndp_[2 * MAX_DIF + 1] = {0};

int canPartition(int* nums, int numsSize) {
    int MAX = 0;
    for (int i = 0; i < numsSize; ++i) {
        MAX += nums[i];
    }

    for (int i = 0; i < 2 * MAX + 1; ++i) {
        dp_[i] = ndp_[i] = 0;
    }

    int* dp = dp_;
    int* ndp = ndp_;

    dp[MAX] = 1;
    for (int i = 0; i < numsSize; ++i) {
        for (int i = 0; i < 2 * MAX + 1; ++i) {
            ndp[i] = 0;
        }

        for (int dif = 0; dif <= 2 * MAX; ++dif) {
            if (dif - nums[i] >= 0) {
                ndp[dif - nums[i]] |= dp[dif];
            }

            if (dif + nums[i] <= 2 * MAX) {
                ndp[dif + nums[i]] |= dp[dif];
            }
        }

        {
            int* t = dp;
            dp = ndp;
            ndp = t;
        }
    }

    return dp[MAX];
}