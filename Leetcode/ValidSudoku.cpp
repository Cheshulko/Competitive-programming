class Solution {
public:

    int have[100];
    bool check(){
        for(int i = 0; i < 100; ++i) if(have[i] > 1) return false;
        return true;
    }

    bool isValidSudoku(vector<vector<char>>& board) {
        int size = board.size();
        
        for(int i = 0; i < size; ++i){
            memset(have, 0, sizeof have);
            for(int j = 0; j < size; ++j){
                if(board[i][j] != '.') have[board[i][j] - '1']++;
            }
            if(!check()) return false;
        } 
        for(int j = 0; j < size; ++j){
            memset(have, 0, sizeof have);
            for(int i = 0; i < size; ++i){
                if(board[i][j] != '.') have[board[i][j] - '1']++;
            }
            if(!check()) return false;
        }
        for(int i = 1; i <= 3; ++i){
            for(int j = 1; j <= 3; ++j){
                memset(have, 0, sizeof have);
                for(int k = i*3-3; k < i*3; ++k){
                    for(int l = j*3-3; l < j*3; ++l){
                        if(board[k][l] != '.') have[board[k][l] - '1']++;
                    }
                }
                if(!check()) return false;
            }
        }
        return true;
    }
};
