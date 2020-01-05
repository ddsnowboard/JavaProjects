import qualified Data.Set as Set
import Data.List
import System.IO

data SpellingBee = SpellingBee { yellow :: Char
                               , otherLetters :: Set.Set Char
                               } deriving Show

filename = "enable1.txt"

baseProblem = SpellingBee 'g' (Set.fromList "lapemx")

numOtherLetters = 6
totalLetters = numOtherLetters + 1

main = do
    handle <- openFile filename ReadMode
    contents <- hGetContents handle
    let words = lines contents
    let dictionary = Set.fromAscList words
    putStr $ show $ score $ matchingWords dictionary baseProblem


matchingWords :: Set.Set String -> SpellingBee -> Set.Set String
matchingWords dictionary problem = Set.filter matchProblem dictionary
    where matchProblem s = (length s >= 4) && (elem center s) && (all ((flip elem) rest) (filter (/=center) s))
          center = yellow problem
          rest = otherLetters problem

score :: Set.Set String -> Int
score words = sum $ Set.map scoreOne $ words

scoreOne :: String -> Int
scoreOne word = bonusPoints + basePoints
    where bonusPoints = if (allUnique word) then totalLetters else 0
          allUnique s = (length s) == (uniqueSetSize s)
          wordLen = length word
          basePoints | wordLen == 4 = 1
            | wordLen >= 5 = wordLen
            | otherwise = 0


uniqueSetSize :: (Ord a) => [a] -> Int
uniqueSetSize = (Set.size . Set.fromList)

allSpellingBees = concat $ map allBeesGivenYellow ['a'..'z']
    where allBeesGivenYellow c = map ((SpellingBee c) . Set.fromList) (combinations numOtherLetters (['a'..'z'] \\ [c]))

combinations :: Int -> [a] -> [[a]]
combinations 0 _ = [[]]
combinations _ [] = []
combinations n (x:xs) = [(x:rest) | rest <- combinations (n - 1) xs] ++ (combinations n xs)

