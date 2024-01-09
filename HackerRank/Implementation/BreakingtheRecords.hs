-- https://www.hackerrank.com/challenges/breaking-best-and-worst-records

import Data.List

solveCase :: ([Int] -> Int) -> [Int] -> Int
solveCase f xs = (length $ group $ map f $ tail $ inits xs) - 1

solve :: [Int] -> [Int]
solve xs = [best, worst]
    where best  = solveCase maximum xs
          worst = solveCase minimum xs

main = interact $ unwords . map show . solve . map read . tail . words

-- 2

foldFnBase :: (Int -> Int -> Bool) -> (Int, Int) -> Int -> (Int, Int)
foldFnBase f (cur,cnt) x
    | f x cur   = (x,cnt+1)
    | otherwise = (cur,cnt) 

foldFn :: (Int -> Int -> Bool) -> [Int] -> Int
foldFn f xs = snd $ foldl (foldFnBase f) (head xs, 0) (tail xs)

solve2 :: [Int] -> [Int]
solve2 xs = [best, worst]
    where best  = foldFn (>) xs
          worst = foldFn (<) xs

main = interact $ unwords . map show . solve2 . map read . tail . words