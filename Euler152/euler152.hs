import Numeric
import Data.Char

top = 30

bottom = 2

invSquare :: Int -> Double
invSquare n = 1.0 / (fromIntegral n)^2

sumInvSquares :: Double -> Int
sumInvSquares goal = sumInvSquares' goal bottom top 0 0

sumInvSquares' :: Double -> Int -> Int -> Double -> Int -> Int
-- sumInvSquares' 0.5 4 6 (0.5 - 1/25) 0 = 1
sumInvSquares' goal curr end sum count 
  | curr > end = count
  | sum + (invSquare curr) < goal = (sumInvSquares' goal (curr + 1) end (sum + (invSquare curr)) count) + (sumInvSquares' goal (curr + 1) end sum count)
  | sum + (invSquare curr) == goal = 1 + (sumInvSquares' goal (curr + 1) end (sum + (invSquare curr)) count) + (sumInvSquares' goal (curr + 1) end sum count)
  | sum + (invSquare curr) > goal = (sumInvSquares' goal (curr + 1) end sum count)


run = sumInvSquares 0.5
main = print run
