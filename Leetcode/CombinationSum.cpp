#include <iostream>
#include <vector>
#include <queue>

using namespace std;

class Solution
{
  private:
    vector<int> cur;
    vector<int> candidates;
    vector<vector<int>> ans;

  public:
    void dfs(int sum, int from, const int &target)
    {
        for (int i = from; i < candidates.size(); ++i)
        {
            int lSum = sum + candidates[i];
            cur.push_back(candidates[i]);
            if (lSum < target)
            {
                dfs(lSum, i, target);
            }
            else if (lSum == target)
            {
                ans.push_back(cur);
            }
            cur.pop_back();
        }
    }
    vector<vector<int>> combinationSum(vector<int> &candidates, int target)
    {
        this->candidates = candidates;
        dfs(0, 0, target);
        return ans;
    }
};

int main()
{
    vector<int> in = {2, 3, 5};

    Solution s;
    auto ans = s.combinationSum2(in, 7);
    for (int i = 0; i < ans.size(); ++i)
    {
        for (int j = 0; j < ans[i].size(); ++j)
        {
            cout << ans[i][j] << " ";
        }
        cout << endl;
    }
    return 0;
}