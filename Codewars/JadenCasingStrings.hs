-- https://www.codewars.com/kata/5390bac347d09b7da40006f6

import Data.Char

toJadenCase :: String -> String
toJadenCase js = unwords $ map (\(x:xs) -> toUpper x:xs) $ words js