// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i

class Solution {
   public:
    int maxFreeTime(int eventTime,
                    int k,
                    vector<int>& startTime,
                    vector<int>& endTime) {
        size_t n = startTime.size();

        vector<int> startTimeExt(n + 2, 0);
        vector<int> endTimeExt(n + 2, 0);

        for (size_t i = 0; i < n; ++i) {
            startTimeExt[i + 1] = startTime[i];
            endTimeExt[i + 1] = endTime[i];
        }

        startTimeExt[n + 1] = eventTime;
        endTimeExt[n + 1] = eventTime;

        vector<int> pref(n + 3, 0);
        for (size_t i = 0; i <= n + 1; ++i) {
            pref[i + 1] += pref[i] + (endTimeExt[i] - startTimeExt[i]);
        }

        int ans = 0;
        for (size_t i = k; i < n + 1; ++i) {
            ans = max(ans, startTimeExt[i + 1] - endTimeExt[i - k] -
                               (pref[i + 1] - pref[i - k + 1]));
        }

        return ans;
    }
};