class Solution {
public:
    string addBinary(string a, string b) {
        string res;
        int la = a.length() - 1, lb = b.length() - 1;
        int up = 0, I = 0;
        while(la >= 0 && lb >= 0){
            int g = a[la--] - '0' + b[lb--] - '0' + up;
            up = g / 2;
            g %= 2;
            res.push_back(g + '0');
        }
        while(la >= 0){
            int g = a[la--] - '0' + up;
            up = g / 2;
            g %= 2;
            res.push_back(g + '0');
        }
        while(lb >= 0){
            int g = b[lb--] - '0' + up;
            up = g / 2;
            g %= 2;
            res.push_back(g + '0');
        }
        
        if(up) res.push_back('1');
        reverse(res.begin(), res.end());
        return res;
    }
};
