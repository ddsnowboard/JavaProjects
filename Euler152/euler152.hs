import Numeric
import Data.Char

top = 20

bottom = 2

inverseSquares = [1.0 / (fromIntegral x)^2 :: Float | x <- [bottom..top]]

toBinString n = (showIntAtBase 2 intToDigit n "")

isSet (i, '1') = i
isSet (i, '0') = 0

combination n = map isSet [x | x <- zip inverseSquares (toBinString n)]

bitLength :: Int -> Integer
bitLength n = (floor(((log(fromIntegral n) :: Float) / (log(2.0) :: Float)) + 1)) :: Integer

bitstrings :: Integer -> Integer -> [Int]
bitstrings minLen maxLen =  [x | x <- (takeWhile (\x -> bitLength x <= maxLen) [0..]), (bitLength x) >= minLen]

combinations = map combination (bitstrings bottom top)

listSumsEq :: (Num a, Ord a) => a -> [a] -> Bool
listSumsEq n xs = listSumsEq' n 0 xs

listSumsEq' s n (x:xs) | s > n = False
  | s <= n = listSumsEq' (s + x) n xs 

listSumsEq' s n [] = s == n

equalHalf :: Int
equalHalf = length (filter (listSumsEq 0.5) combinations)


main = do 
    print equalHalf
