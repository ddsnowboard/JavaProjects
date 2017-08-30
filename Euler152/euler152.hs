import Numeric
import Data.Char

top = 5

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

sums :: [Float]
sums = map sum combinations

delta = 0.1
half = 0.5
nearHalf :: Float -> Bool
nearHalf x = (abs x - half) < delta

main = do 
    print (length [x | x <- sums, nearHalf x])
