import Data.Maybe
main = do 
    input <- getLine
    let output = evalRpn input
    case output of
        Just n -> putStrLn $ (show n)
        Nothing -> putStrLn "You made an error somewhere"
    

data Token = Add | Subtract | Multiply | Divide | Number Double
            

evalRpn :: String -> Maybe Double
evalRpn input = let 
    tokenStrings = words input
    tokens = map tokenize tokenStrings
    in
    eval tokens []
    

eval :: [Token] -> [Double] -> Maybe Double
eval [] (x:xs) = Just x
eval (Add:xs) (a:b:rest) = eval xs (a+b:rest)
eval (Subtract:xs) (a:b:rest) = eval xs (b-a:rest)
eval (Multiply:xs) (a:b:rest) = eval xs (a * b:rest)
eval (Divide:xs) (a:b:rest) = eval xs (b/a:rest)
eval (Number n:xs) stack = eval xs (n:stack)
eval [] [] = Just 0
eval _ _ = Nothing

tokenize :: String -> Token
tokenize "+" = Add
tokenize "-" = Subtract
tokenize "*" = Multiply
tokenize "/" = Divide
tokenize n = Number (read n)
