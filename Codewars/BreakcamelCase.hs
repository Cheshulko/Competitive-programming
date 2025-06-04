-- https://www.codewars.com/kata/5208f99aee097e6552000148

module Codewars.Kata.BreakCamelCase (solution) where

import Data.Char

solution :: String -> String
solution str = solve str True
    where solve str first
            | str == "" = ""
            | (not first) && (isUpper $ head str) = ' ' : merge str
            | otherwise = merge str
                where merge str = head str : solve (tail str) False