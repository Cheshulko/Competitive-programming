import Data.List

convert :: Char -> Int
convert 'U' = 1
convert 'D' = -1

solve :: [Char] -> Int
solve xs = length $ 
           filter (all (< 0)) $ 
           groupBy (\x y -> x /= 0 && y /= 0) $ 
           scanl (+) 0 (map convert xs)

main = interact $ show . solve . head . tail . words