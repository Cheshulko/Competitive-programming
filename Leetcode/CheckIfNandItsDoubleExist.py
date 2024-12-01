# https://leetcode.com/problems/check-if-n-and-its-double-exist

from typing import  List

class Solution:
    def checkIfExist(self, arr: List[int]) -> bool:
        return any([
            True 
            for i, x in enumerate(arr) 
            for y in arr[:i] 
            if x == 2 * y or y == 2 * x
        ])