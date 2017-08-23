import Numeric
import Data.Char

top = 80

bottom = 2

inverseSquares = [1 `div` x^2 | x <- [bottom..top]]

inInverseSquares x = x `elem` inverseSquares

toBinString n = (showIntAtBase 2 intToDigit n "")

isSet (_, "1") = True
isSet (_, "0") = False

combination n = [x | x <- zip inverseSquares (toBinString n), isSet x]

# Check that the length is greater than or equal to 2 and less than or equal to 80, 
# then you should have all the numbers to turn into bit strings
bitstrings = [x | x <- [0..], log(x) / log(2)

combinations = map combination bitstrings
