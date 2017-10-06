class Solution {
public:
    int maxArea(vector<int>& h) {
        int ans = 0;
        int I = 0, J = h.size() - 1;
        if(J <= 0) return 0;
        while(I < J){
            ans = max(ans, min(h[I], h[J]) * (J - I));
            if(h[I] < h[J]) ++I;
            else --J;
        }
        return ans;
    }
};
