# https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence

class Solution:
    def isPrefixOfWord(self, sentence: str, searchWord: str) -> int:
        return next(iter(
            ind + 1
            for ind, word in enumerate(sentence.split(" "))
            if word.startswith(searchWord)
        ), -1)
        