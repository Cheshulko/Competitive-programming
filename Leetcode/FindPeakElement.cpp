class Solution {
public:
    int findPeakElement(vector<int>& arr) {
        int I = 0, J = arr.size();
        if(J == 1) return 0;
        if(J == 2){
            if(arr[0] > arr[1]) return 0;
            else return 1;
        } 
        if(arr[0] > arr[1]) return 0;
        for(int i = 1; i < J - 1; ++i){
            if(arr[i - 1] < arr[i] && arr[i] > arr[i + 1]) return i;
        }
        return J - 1;
    }
};
