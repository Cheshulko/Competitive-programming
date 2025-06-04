module Codewars.Kata.YourOrderPlease (yourOrderPlease) where

import           Data.Char     (isNumber)
import           Data.Function (on)
import           Data.List     (sortBy)

yourOrderPlease :: String -> String
yourOrderPlease = unwords . sortBy sortFn . words
    where
        sortFn :: String -> String -> Ordering
        sortFn = compare `on` (head . filter isNumber)

