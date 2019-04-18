class Solution {
   public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        if (intervals.empty()) return {};

        sort(intervals.begin(), intervals.end(),
             [&](vector<int>& f, vector<int>& s) {
                 return (f[0] < s[0]) || (f[0] == s[0] && f[1] < s[1]);
             });

        vector<vector<int>> ans;

        int start = intervals[0][0];
        int end = intervals[0][1];
        for (auto& x : intervals) {
            if (end < x[0]) {
                ans.push_back({start, end});
                start = x[0];
            }
            if (x[1] > end) {
                end = x[1];
            }
        }
        ans.push_back({start, end});
        return ans;
    }
};