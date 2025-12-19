// https://leetcode.com/problems/find-all-people-with-secret

class Solution {
   public:
    vector<int> findAllPeople(int n,
                              vector<vector<int>>& meetings,
                              int firstPerson) {
        vector<vector<pair<size_t, int>>> connections(
            n, vector<pair<size_t, int>>());

        connections[0].push_back({firstPerson, 0});
        for (const auto& meeting : meetings) {
            connections[meeting[0]].push_back({meeting[1], meeting[2]});
            connections[meeting[1]].push_back({meeting[0], meeting[2]});
        }

        vector<int> know_at;
        Solution::dijkstra(0, connections, know_at);

        vector<int> ans;
        for (size_t i = 0; i < (size_t)n; ++i) {
            if (know_at[i] != std::numeric_limits<int>::max()) {
                ans.push_back(i);
            }
        }

        return ans;
    }

    void dijkstra(size_t s,
                  const vector<vector<pair<size_t, int>>>& connections,
                  vector<int>& know_at) {
        constexpr auto INF = std::numeric_limits<int>::max();

        size_t n = connections.size();
        know_at.assign(n, INF);
        know_at[s] = -1;

        set<pair<int, size_t>> st;
        st.insert({know_at[s], s});

        while (!st.empty()) {
            size_t v = st.begin()->second;
            st.erase(st.begin());

            for (auto [to, time] : connections[v]) {
                if (know_at[v] <= time && time < know_at[to]) {
                    auto it = st.find({know_at[to], to});
                    if (it != st.end()) {
                        st.erase(it);
                    }

                    know_at[to] = time;
                    st.insert({know_at[to], to});
                }
            }
        }
    }
};