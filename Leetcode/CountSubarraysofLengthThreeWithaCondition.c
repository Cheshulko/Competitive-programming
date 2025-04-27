// https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition

int countSubarrays(int* nums, int numsSize) {
    int ans = 0;
    int arr[] = {nums[0], nums[1], nums[2]};

    for (int i = 2; i < numsSize; ++i) {
        arr[2] = nums[i];
        ans += (arr[1] == 2 * (arr[0] + arr[2]));

        arr[0] = arr[1];
        arr[1] = arr[2];
    }

    return ans;
}