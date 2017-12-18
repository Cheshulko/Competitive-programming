class Solution {
public:
    int gcd(int a, int b){
        while(b != 0){
            a %= b;
            swap(a, b);
        }
        return a;
    }

    void rotate(vector<int>& nums, int k) {
        int len = nums.size();
        k %= len;
        if(len == 0 || len == 1 || k == 0) return;
        
        int lenCycle, cntCycles;
        cntCycles = gcd(len, k);
        lenCycle = len / cntCycles;

        int tmp;
        for(int i = 0; i < cntCycles; ++i){
            tmp = nums[i];
            for(int j = 0; j < lenCycle; ++j){
                int next = (i + (j+1)*k) % len; 
                swap(tmp, nums[next]);
            }
        }
    }
};
