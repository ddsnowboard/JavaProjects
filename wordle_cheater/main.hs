import System.IO
import qualified Data.Map
import Data.Map (fromList, (!))
import Debug.Trace
import Data.Ord
import Data.List
import Control.Monad

main = withFile "someWords.txt" ReadMode (\f -> do
    fileContent <- hGetContents f
    let words = lines fileContent
    print $ findFirstWord words
                                                 )
findFirstWord dictionary = maximumBy (comparing $ (!) memo) possibleWords
    where memo = fromList $ map (\x -> (x, wordsEliminated x)) possibleWords
          rightSize l = 5 == (length l)
          possibleWords = filter rightSize dictionary
          wordsEliminated guess = trace "1" $ sum $ (wordsEliminatedGivenActualWord guess) <$> possibleWords
          wordsEliminatedGivenActualWord guess actualWord = trace "2" $ length $ filter (guessEliminatesWord $ gradeGuess actualWord guess) possibleWords



guessEliminatesWord squares possibleWord = trace "4" $ matches possibleWord squares

matches :: String -> [Square] -> Bool
matches word squares = all (uncurry matches) $ zip word squares
    where matches letter square = case square of
                                    Green c -> letter == c
                                    Yellow c -> any (==c) word
                                    Grey c -> all (/=c) word

gradeGuess actualWord guess = trace "3" $ map (uncurry grade) pairs
    where grade actualLetter guessedLetter = let 
                                                   color = if actualLetter == guessedLetter then Green
                                                   else if any (guessedLetter==) actualWord then Yellow
                                                                                 else Grey
                                                in color guessedLetter 
          pairs = zip actualWord guess

data Square = Grey Char | Yellow Char | Green Char
    deriving Show
