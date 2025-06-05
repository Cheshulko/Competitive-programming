-- https://www.codewars.com/kata/52c31f8e6605bcc646000082

module TwoSum (twoSum) where

import qualified Data.Map as M

twoSum :: [Int] -> Int -> (Int, Int)
twoSum nums target =
    head $
    snd $
    foldr (\(x, ind) (b, ans) -> (M.insert x ind b, insert' ans ind $ M.lookup (target - x) b)) (M.empty, []) $
    zip nums [0..]
    where
        insert' l ind1 (Just ind2) = (ind1, ind2) : l
        insert' l ind1 Nothing     = l
