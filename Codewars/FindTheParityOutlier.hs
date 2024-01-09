-- https://www.codewars.com/kata/5526fc09a1bbd946250002dc

import Data.Maybe
import Data.List

findOutlier :: [Int] -> Int
findOutlier xs 
    | odds  == 1 = fromJust $ find odd xs
    | evens == 1 = fromJust $ find even xs
    where evens = length $ filter even xs
          odds  = length $ filter odd xs