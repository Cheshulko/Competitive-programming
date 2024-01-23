import Data.List
import Data.Function

-- sortBy (\xx xy -> compare (length xy) (length xx)) $ group $ sort [1, 4, 4, 4, 5, 3]
-- sortBy (\xx xy -> flip compare (length xx) (length xy)) $ group $ sort [1, 4, 4, 4, 5, 3]
-- sortBy (flip compare `on` length) $ group $ sort [1, 4, 4, 4, 5, 3]

solve :: [Int] -> Int
solve xs = head $ head $ sortBy (flip compare `on` length) $ group $ sort xs

main = interact $ show . solve . map read . tail . words