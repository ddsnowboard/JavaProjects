main = print findMaxSum

allPowers a b = [a'^b' | a' <- [1..a], b' <- [1..b]] 

findMaxSum :: Int
findMaxSum = maximum (map sumDigits (allPowers 100 100))

-- I'm guessing that if I make this faster, it might work. I should skip the getDigits step
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
