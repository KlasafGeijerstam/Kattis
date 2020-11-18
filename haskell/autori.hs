import Data.Char
main = do
    q <- getLine
    putStrLn $ filter isUpper q
