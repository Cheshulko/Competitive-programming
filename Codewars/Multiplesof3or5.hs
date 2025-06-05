-- https://www.codewars.com/kata/514b92a657cdc65150000006

module MultiplesOf3And5 where

import           Control.Applicative (Applicative (liftA2))

solution :: Integer -> Integer
solution number = sum $ filter ((== 0) . ((*) <$> (`mod` 3) <*> (`mod` 5))) [1..number - 1]
