solve :: [Int] -> Int
solve [n,k] = min (k `div` 2) ((n `div` 2) - (k `div` 2))

main = interact $ show . solve . map read . words