-- https://www.codewars.com/kata/54da539698b8a2ad76000228

module Codewars.Kata.TenMinuteWalk where

isValidWalk :: [Char] -> Bool
isValidWalk walk = len == 10 && n == s && e == w 
    where len       = length $ trimmed
          n         = length $ filter (== 'n') $ trimmed
          s         = length $ filter (== 's') $ trimmed
          e         = length $ filter (== 'e') $ trimmed
          w         = length $ filter (== 'w') $ trimmed
          trimmed   = take 11 walk