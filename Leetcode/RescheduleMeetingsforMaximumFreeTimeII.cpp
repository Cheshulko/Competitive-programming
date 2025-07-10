// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii

class Solution {
   public:
    int maxFreeTime(int eventTime,
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

        int ans = 0;
        {
            const size_t k = 1;

            vector<int> pref(n + 3, 0);
            for (size_t i = 0; i <= n + 1; ++i) {
                pref[i + 1] += pref[i] + (endTimeExt[i] - startTimeExt[i]);
            }

            for (size_t i = k; i < n + 1; ++i) {
                ans = max(ans, startTimeExt[i + 1] - endTimeExt[i - k] -
                                   (pref[i + 1] - pref[i - k + 1]));
            }
        }

        {
            using Slot = pair<int, size_t>;

            set<Slot> slots;
            for (size_t i = 0; i < n + 1; ++i) {
                slots.insert({startTimeExt[i + 1] - endTimeExt[i], i});
            }

            for (size_t i = 0; i < n; ++i) {
                auto event = endTimeExt[i + 1] - startTimeExt[i + 1];

                Slot slot1 = {startTimeExt[i + 1] - endTimeExt[i], i};
                Slot slot2 = {startTimeExt[i + 2] - endTimeExt[i + 1], i + 1};

                slots.erase(slots.find(slot1));
                slots.erase(slots.find(slot2));

                if (slots.size() > 0) {
                    auto best = --slots.end();

                    if (best->first >= event) {
                        ans = max(ans, startTimeExt[i + 2] - endTimeExt[i]);
                    }
                }

                slots.insert(slot1);
                slots.insert(slot2);
            }
        }

        return ans;
    }
};