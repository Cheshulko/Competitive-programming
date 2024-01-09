solve :: [Int] -> [Int] -> Int
solve as bs = length 
                $ filter (\x -> bsGcd `mod` x == 0) 
                $ takeWhile (<= bsGcd) 
                $ map (* asLcm) [1 .. ]
    where asLcm = foldl1 lcm as
          bsGcd = foldl1 gcd bs

readIntList :: IO [Int]
readIntList = 
    do line <- getLine
       return $ map read $ words line

main = do [n, m] <- readIntList
          as     <- readIntList
          bs     <- readIntList
          putStrLn $ show $ solve as bs
