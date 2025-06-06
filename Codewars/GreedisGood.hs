-- https://www.codewars.com/kata/5270d0d18625160ada0000e4

module Greed (score) where

import           Data.List (group, sort)

score :: [Int] -> Int
score dice = solve 0 scores
    where
        scores = map (\l -> (head l, length l)) $ group $ sort dice

        solve ans [] = ans
        solve ans xs = solve (ans + fst sxs) (snd sxs)
            where
                sxs = count' xs

        count' ((_, 0) : xs) = (0, xs)

        count' ((1, x) : xs)
            | x >= 3    = (1000, (1, x - 3) : xs)
            | otherwise = (100,  (1, x - 1) : xs)

        count' ((2, x) : xs)
            | x >= 3    = (200,  (2, x - 3) : xs)
            | otherwise = (0,    (2, x - 1) : xs)

        count' ((3, x) : xs)
            | x >= 3    = (300,  (3, x - 3) : xs)
            | otherwise = (0,    (3, x - 1) : xs)

        count' ((4, x) : xs)
            | x >= 3    = (400,  (4, x - 3) : xs)
            | otherwise = (0,    (4, x - 1) : xs)

        count' ((5, x) : xs)
            | x >= 3    = (500,  (5, x - 3) : xs)
            | otherwise = (50,   (5, x - 1) : xs)

        count' ((6, x) : xs)
            | x >= 3    = (600,  (6, x - 3) : xs)
            | otherwise = (0,    (6, x - 1) : xs)

--  Three 1's => 1000 points
--  Three 6's =>  600 points
--  Three 5's =>  500 points
--  Three 4's =>  400 points
--  Three 3's =>  300 points
--  Three 2's =>  200 points
--  One   1   =>  100 points
--  One   5   =>   50 point
