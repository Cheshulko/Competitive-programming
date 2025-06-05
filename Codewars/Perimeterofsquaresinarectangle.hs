-- https://www.codewars.com/kata/559a28007caad2ac4e000083

module Codewars.Kata.Perimeter (perimeter) where

perimeter :: Integer -> Integer
perimeter = (* 4) . sum . flip take fibs . (+ 1) . fromInteger
    where
        fibs = 1 : scanl (+) 1 fibs
