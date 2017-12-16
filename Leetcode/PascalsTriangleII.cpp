class Solution {
public:
    vector<int> getRow(int rowIndex) {
        vector<int> a, b;
        a.push_back(1);
        for(int i = 0; i < rowIndex - 1; ++i){
            b.clear();
            b.push_back(1);
            for(int j = 1; j < a.size(); ++j){
                b.push_back(a[j - 1] + a[j]);
            }
            if(i % 2 == 0) b.push_back(2 * a[(int)a.size() - 1]);
            swap(a, b);
        }
        b = a;
        int st = (rowIndex % 2 == 1 ? a.size() - 1 : a.size() - 2);
        for(; st >= 0; --st) a.push_back(b[st]);
        return a;
    }
};
