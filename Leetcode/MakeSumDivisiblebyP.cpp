// https://leetcode.com/problems/make-sum-divisible-by-p

class Solution {
   public:
    int minSubarray(vector<int>& nums, int p) {
        using ll = long long;

        const size_t n = nums.size();

        ll sum = 0;
        for (size_t i = 0; i < n; ++i) {
            sum += nums[i];
        }

        map<ll, size_t> max_pref;
        max_pref[0] = 0;

        ll pref = 0;
        size_t ans = n;
        for (size_t i = 0; i < n; ++i) {
            pref += nums[i];

            const ll rest = sum - pref;
            const ll extra = rest % p;

            if (pref % p == 0) {
                ans = min(ans, n - i - 1);
            }
            if (max_pref.cend() != max_pref.find((p - extra) % p)) {
                ans = min(ans, i + 1 - max_pref[(p - extra) % p]);
            }

            max_pref[pref % p] = i + 1;
        }

        return ans == n ? -1 : ans;
    }
};