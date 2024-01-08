getMiddle :: String -> String
getMiddle s
    | n == 0    = []
    | odd n     = [s !! n2]
    | otherwise = [s !! (n2 - 1), s !! n2]
    where n  = length s
          n2 = (n `div` 2) 