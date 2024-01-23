solve :: [Int] -> String
solve (_:k:xs)
    | l - r == 0 = "Bon Appetit"
    | otherwise  = show $ l - r 
    where l = last xs
          t = xs !! k
          r = (sum xs - l - t) `div` 2

-- main = interact $ solve . map read . words

-----------------------------------------------------

excludeNth :: Int -> [a] -> [a]
excludeNth n xs = left ++ tail right
    where (left, right) = splitAt n xs

-- ghci> getList :: IO [Int]
-- 1 2 3 4 5
getList :: Read a => IO [a]
getList = do
    line <- getLine
    return $ map read $ words line


solve2:: Int -> [Int] -> Int -> Maybe Int
solve2 k bill b 
    | b > actualPrice = Just (b - actualPrice)
    | otherwise       = Nothing
    where actualPrice = (sum $ excludeNth k bill) `div` 2

main :: IO ()
main = do 
    -- import Control.Monad
    -- [[_, k], bill, [b]] <- replicateM 3 getList
    [_, k] <- getList
    bill   <- getList
    [b]    <- getList
    putStrLn $ maybe "Bon Appetit" show (solve2 k bill b) 
