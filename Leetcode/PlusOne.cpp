class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        int up = 1;
        int l = digits.size();
        vector<int> res;
        for(int i = l - 1; i >= 0; --i){
            int g = digits[i] + up;
            up = g / 10;
            g %= 10;
            res.push_back(g);
        }
        if(up) res.push_back(1);
        reverse(res.begin(), res.end());
        return res;
    }
};
