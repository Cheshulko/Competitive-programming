module LeapYear (isLeapYear) where

isLeapYear :: Integer -> Bool
isLeapYear year
  | rem year 400 == 0 = True
  | rem year 100 == 0 = False
  | rem year 4 == 0 = True
  | otherwise = False
