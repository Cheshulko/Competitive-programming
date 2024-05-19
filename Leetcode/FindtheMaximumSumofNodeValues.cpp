class Solution {
   public:
    long long inf = 9999999999999999;
    void dfs(int cur,
             int k,
             const vector<vector<int>>& adj,
             const vector<int>& nums,
             vector<array<long long, 2>>& dp,
             vector<bool>& used) {
        used[cur] = true;

        auto csize = adj[cur].size();
        auto cdp_prev = array<long long, 2>{0, -inf};
        auto cdp = array<long long, 2>{0, 0};

        for (int i = 0; i < csize; ++i) {
            auto to = adj[cur][i];
            if (used[to]) {
                continue;
            } else {
                dfs(to, k, adj, nums, dp, used);

                cdp[0] = max(cdp_prev[0] + dp[to][0], cdp_prev[1] + dp[to][1]);
                cdp[1] = max(cdp_prev[1] + dp[to][0], cdp_prev[0] + dp[to][1]);

                swap(cdp, cdp_prev);
            }
        }

        dp[cur][0] =
            max(cdp_prev[1] + (nums[cur] ^ k), cdp_prev[0] + nums[cur]);
        dp[cur][1] =
            max(cdp_prev[0] + (nums[cur] ^ k), cdp_prev[1] + nums[cur]);
    }

    long long maximumValueSum(vector<int>& nums,
                              int k,
                              vector<vector<int>>& edges) {
        int n = nums.size();

        vector<vector<int>> adj(n, vector<int>());
        for (int i = 0; i < edges.size(); ++i) {
            auto u = edges[i][0];
            auto v = edges[i][1];

            adj[u].push_back(v);
            adj[v].push_back(u);
        }

        vector<array<long long, 2>> dp(n, {0, 0});
        vector<bool> used(n, false);

        dfs(0, k, adj, nums, dp, used);

        return dp[0][0];
    }
};