// https://leetcode.com/problems/find-words-containing-character

int* findWordsContaining(char** words, int wordsSize, char x, int* returnSize) {
    int* ans = (int*)malloc(wordsSize * sizeof(int));
    *returnSize = 0;

    for (int j = 0; j < wordsSize; ++j) {
        if (strchr(words[j], x)) {
            ans[(*returnSize)++] = j;
        }
    }

    return ans;
}