-- If I type what main is into the interpreter, it works fine, but I can't just run main. I'm so confused.
-- main = print (findMaxSum 100)
main = let 
        sums = digitSums 100
     in
        print $ maximum sums

allPowers a b = [a'^b' | a' <- [1..a], b' <- [1..b]] 

digitSums n = map sumDigits $ allPowers n n

sumDigits :: (Integral a) => a -> a
sumDigits n = sumDigits' n 0

sumDigits' 0 acc = acc
sumDigits' n acc = sumDigits' (n `div` 10) (acc + (n `mod` 10))
