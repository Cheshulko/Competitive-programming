round5 :: Int -> Int
round5 x 
    | (m5 - x) < 3 && x >= 38 = m5
    | otherwise               = x
    where m5 = x + (5 - x `mod` 5)
    
solve :: [Int] -> [Int]
solve xs = map round5 xs

main = interact $ unlines . map show . solve . map read . tail . words