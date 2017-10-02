class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& arr) {
      sort(arr.begin(), arr.end());
      int n = arr.size();
      vector<vector<int>> ans;
      
      if(arr.size() < 3) return ans;
      int previ = -124124, prevj = -123123;
      for(int i = 0; i < n - 2; ++i){
          while(previ == arr[i]) ++i;
          prevj = -12312312;
          for(int j = i + 1; j < n - 1; ++j){
              while(prevj == arr[j]) ++j;
              
              int k = j + 1; 
              while(k < n && arr[i] + arr[j] + arr[k] <= 0){
                  if(arr[i] + arr[j] + arr[k] == 0){
                      vector<int> g;
                      g.push_back(arr[i]);
                      g.push_back(arr[j]);
                      g.push_back(arr[k]);
                      ans.push_back(g);
                      break;
                  }
                  else ++k;
              }
              prevj = arr[j];
          }
          previ = arr[i];
      }
      return ans;
    }
    
};
