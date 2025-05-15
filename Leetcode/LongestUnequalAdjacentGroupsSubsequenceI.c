// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i

char** getLongestSubsequence(char** words,
                             int wordsSize,
                             int* groups,
                             int groupsSize,
                             int* returnSize) {
    *returnSize = 0;
    int cur = -1;
    for (int i = 0, j = 0; i < wordsSize; ++i) {
        if (cur != groups[i]) {
            words[(*returnSize)++] = words[i];
            cur = groups[i];
        }
    }

    return words;
}