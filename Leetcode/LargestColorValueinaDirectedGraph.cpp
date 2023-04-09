// https://leetcode.com/problems/largest-color-value-in-a-directed-graph

#include <cmath>
#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;

struct DirectedGraph {
    vector<vector<int>> edges;
    string colors;

    DirectedGraph(vector<vector<int>>& edges, string& colors) {
        vector<vector<int>> edges_tr(colors.length(), vector<int>());
        for (auto& edge : edges) {
            edges_tr[edge[0]].push_back(edge[1]);
        }
        this->edges = edges_tr;
        this->colors = colors;
    }

    vector<int> find_starts() {
        vector<int> edges_in(colors.length(), 0);
        for (int node_from = 0; node_from < this->edges.size(); ++node_from) {
            auto& edges = this->edges[node_from];
            for (auto& node_to : edges) {
                if (node_to == node_from) {
                    return {};
                }
                edges_in[node_to] += 1;
            }
        }
        vector<int> ans;
        for (int node = 0; node < colors.length(); node++) {
            if (edges_in[node] == 0) {
                ans.push_back(node);
            }
        }

        return ans;
    }

    void dfs(int parent,
             int cur,
             vector<vector<int>>& paths,
             vector<int>& parents) {
        int color = colors[cur] - 'a';

        if (this->edges[cur].size() == 0) {
            paths[cur][color] = 1;
        }

        // set parent for cur
        parents[cur] = parent;

        for (auto& node_to : this->edges[cur]) {
            if (parents[node_to] == -1) {
                dfs(cur, node_to, paths, parents);
            }

            for (int lcolor = 0; lcolor < 30; lcolor++) {
                paths[cur][lcolor] =
                    max(paths[cur][lcolor],
                        paths[node_to][lcolor] + (lcolor == color ? 1 : 0));
            }
        }
    }

    bool has_cycle(vector<bool>& visited, vector<bool>& recStack, int u) {
        visited[u] = true;
        recStack[u] = true;

        for (int v : edges[u]) {
            if (!visited[v] && has_cycle(visited, recStack, v)) {
                return true;
            } else if (recStack[v]) {
                return true;
            }
        }

        recStack[u] = false;
        return false;
    }

    static int largest_path_value(string& colors, vector<vector<int>>& edges) {
        int nodes = colors.length();
        DirectedGraph graph(edges, colors);

        vector<bool> visited(nodes, false);
        vector<bool> recStack(nodes, false);

        for (int i = 0; i < nodes; ++i) {
            if (!visited[i]) {
                if (graph.has_cycle(visited, recStack, i)) {
                    return -1;
                }
            }
        }

        vector<int> starts = graph.find_starts();

        if (starts.size() > 0) {
            vector<vector<int>> paths(nodes, vector<int>(30, 0));
            vector<int> parents(nodes, -1);
            int ans = 0;
            for (auto& start : starts) {
                graph.dfs(-2, start, paths, parents);
                for (int lcolor = 0; lcolor < 30; lcolor++) {
                    ans = max(ans, paths[start][lcolor]);
                }
            }
            for (int i = 0; i < nodes; ++i) {
                if (parents[i] == -1) {
                    return -1;
                }
            }
            return ans;
        } else {
            return -1;
        }
    }
};

class Solution {
   public:
    int largestPathValue(string colors, vector<vector<int>>& edges) {
        return DirectedGraph::largest_path_value(colors, edges);
    }
};