solve :: [Int] -> [Int]
solve (s:t:a:b:m:_:rest) = [apples, oranges]
    where apples  = length $ filter (\x -> s <= x && x <= t) $ map (\x -> x + a) $ take m rest
          oranges = length $ filter (\x -> s <= x && x <= t) $ map (\x -> x + b) $ drop m rest 

main = interact $ unlines . map show . solve . map read . words