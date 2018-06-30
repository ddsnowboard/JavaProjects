import Data.Maybe
main = do 
    input <- getLine
    putStrLn $ show (evalRpn input)

data Token = Add | Subtract | Multiply | Divide | Number Double
            

evalRpn :: String -> Double
evalRpn input = let 
    tokenStrings = words input
    tokens = map tokenize tokens
    in
    eval tokens []

eval :: [Token] -> [Double] -> Maybe Double
eval [] (x:xs) = Just x
eval (Add:xs) (a:b:rest) = eval xs (a+b:rest)
eval (Subtract:xs) (a:b:rest) = eval xs (a-b:rest)
eval (Multiply:xs) (a:b:rest) = eval xs (a * b:rest)
eval (Divide:xs) (a:b:rest) = eval xs (a / b:rest)
eval (Number n:xs) stack = eval xs (n:stack)
eval [] [] = 0
eval _ _ = Nothing

tokenize :: String -> Token
tokenize "+" = Add
tokenize "-" = Subtract
tokenize "*" = Multiply
tokenize "/" = Divide
tokenize n = Number (read n)
