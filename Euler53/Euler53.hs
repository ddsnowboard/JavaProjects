module Main where
    million = 1000000
    mulRangeInclusive a b = foldl (*) 1 [a..b]

    fact 0 = 1
    fact n = mulRangeInclusive 1 n

    combinations n r = (mulRangeInclusive (r + 1) n) `div` (fact (n - r))

    forAllRs n = map (combinations n) [1..n]

    getBigs [] = []
    getBigs (x:xs)
      | x >= million = x:(getBigs xs)
      | otherwise = getBigs xs


    allCombos n = map forAllRs [1..n]

    bigCombos l = map getBigs l

    countCombosMoreThanMillion = sum (map length (bigCombos (allCombos 100)))

    main = print countCombosMoreThanMillion
