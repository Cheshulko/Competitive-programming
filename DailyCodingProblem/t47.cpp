#include <iostream>
#include <vector>
using namespace std;

class Solution
{
  public:
    int maxProfit(vector<int> &prices)
    {
        if (prices.size() == 0)
            return 0;
        int mn = prices[0];
        int mx = prices[0];
        int ans = 0;
        for (const auto &p : prices)
        {
            mx = max(mx, p);
            ans = max(ans, mx - mn);
            if (p < mn)
            {
                mn = p;
                mx = p;
            }
        }
        return ans;
    }
};

int main()
{
    Solution s;
    vector<int> v = {2, 1, 8, 5, 7, 10};
    cout << s.maxProfit(v) << endl;
    return 0;
}