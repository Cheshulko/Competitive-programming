-- https://www.codewars.com/kata/54b42f9314d9229fd6000d9c

module Dups where

import           Data.Char (toLower)

duplicateEncode :: String -> String
duplicateEncode str = map (mapper . cnt . toLower) str
    where
        cnt c = length $ filter ((c ==) . toLower) str
        mapper 1 = '('
        mapper _ = ')'
