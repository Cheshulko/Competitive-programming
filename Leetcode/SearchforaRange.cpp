class Solution {
public:
    vector<int> searchRange(vector<int>& arr, int target) {
        int R = arr.size(), L = 0;
        vector<int> ans;
        if(R==0){
            vector<int> a;
            a.push_back(-1); a.push_back(-1);
            return a;
        }
        int l = 0, r = R - 1;
        while(r - l > 1){
            int m = (l + r) / 2;
            if(arr[m] < target) l = m;
            else r = m;
        }
        if(arr[l] == target) ans.push_back(l);
        else if(arr[r] == target) ans.push_back(r);
        else {
            vector<int> a;
            a.push_back(-1); a.push_back(-1);
            return a;
        }        
        l = 0, r = R - 1;
        while(r - l > 1){
            int m = (l + r) / 2;
            if(arr[m] <= target) l = m;
            else r = m;
        }
        if(arr[r] == target) ans.push_back(r);
        else ans.push_back(l);
        return ans;        
    }
};
