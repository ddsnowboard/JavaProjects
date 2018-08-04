main = print $ show findMaxSum

findMaxSum :: Int
findMaxSum = maximum (map sumDigits [a^b | a <- [1..100], b <- [1..100]])

sumDigits :: (Integral a) => a -> a
sumDigits n = sum (getDigits n)

getDigits :: (Integral b) => b -> [b]
getDigits n = getDigits' n []

getDigits' :: (Integral c) => c -> [c] -> [c]
getDigits' 0 l = l
getDigits' n l = 
            let 
                divided = n `div` 10
                onesDigit = n `mod` 10
             in
                getDigits' divided (onesDigit:l)
