class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        vector< vector<int> > ans;
        if(numRows == 0) return ans;
        
        vector<int> a, b;
        a.push_back(1); 
        ans.push_back(a);
        
        for(int i = 1; i < numRows; ++i){
            b.clear();
            b.push_back(1);
            for(int j = 1; j < a.size(); ++j){
                b.push_back(a[j-1] + a[j]);
            }
            b.push_back(1);
            swap(a, b);
            ans.push_back(a);
        }
        return ans;
        
    }
};
