class Solution {
public:
    int *used;
    bool cycle;    
    vector<int> out;
    
    void dfs(int v, vector<pair<int, int>>& g){
        used[v] = 1;
        for(int i = 0; i < g.size(); ++i){
            int to = -1;
            if(g[i].second == v) to = g[i].first;
            if(to != -1){
                if(!used[to]) dfs(to, g);                
            }
        }
        out.push_back(v);
    }

    bool canFinish(int numCourses, vector<pair<int, int>>& g) {
        used = new int[numCourses];
        for (int i = 0; i < numCourses; ++i) used[i] = 0;
        cycle = false;
        for(int i = 0; i < numCourses; ++i){
            if(!used[i]) dfs(i, g);
        }
        reverse(out.begin(), out.end());
        map<int, int> gg;
	    for (int i = 0; i < out.size(); ++i) gg[out[i]] = i;
	    for (int i = 0; i < g.size(); ++i){
		    if (gg[g[i].second] > gg[g[i].first]) return false;
	    }
	    return true;
    }
};
