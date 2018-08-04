main = print findMaxSum

findMaxSum :: Int
findMaxSum = max (map sumDigits [a^b | a <- [1..100], b <- [1..100]])

sumDigits :: (Num a) a -> Int
sumDigits n = sum (getDigits n)

getDigits :: (Num a) a -> [Int]
