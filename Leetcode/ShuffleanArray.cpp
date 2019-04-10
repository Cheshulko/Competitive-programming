class Solution {
public:
    Solution(vector<int>& nums) {
        _v = nums;
        srand(time(0));
    }
    
    /** Resets the array to its original configuration and return it. */
    vector<int> reset() {
        return _v;
    }
    
    /** Returns a random shuffling of the array. */
    vector<int> shuffle() {
        if(_v.empty()) return _v;
        vector<int> res;
        res.push_back(_v[0]);
        for (int i = 1; i < _v.size(); ++i){
            res.push_back(_v[i]);
            int ind = (int)(rand() % (i + 1));
            swap(res[ind], res[i]);
        }
        return res;
    }
    
    vector<int> _v;
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(nums);
 * vector<int> param_1 = obj->reset();
 * vector<int> param_2 = obj->shuffle();
 */