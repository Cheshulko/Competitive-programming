// https://leetcode.com/problems/minimum-time-visiting-all-points

class Solution {
   public:
    int minTimeToVisitAllPoints(vector<vector<int>>& points) {
        int ans = 0;
        int x = points[0][0];
        int y = points[0][1];

        for (int i = 1; i < points.size(); ++i) {
            int xto = points[i][0];
            int yto = points[i][1];

            ans += max(abs(x - xto), abs(y - yto));
            x = xto, y = yto;
        }

        return ans;
    }
};