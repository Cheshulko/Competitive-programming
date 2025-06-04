-- https://www.codewars.com/kata/5592e3bd57b64d00f3000047

module Codewars.Kata.PileOfCubes where

findNb :: Integer -> Integer
findNb x = next x 1
    where next need n 
            | need == 0   = n - 1
            | need < n3   = -1
            | otherwise   = next (need - n3) (n + 1)
            where n3 = n * n * n