fizzBuzz :: [Int] -> [String]
fizzBuzz x
    | x !! 0 > x !! 3 = []
    | mo && ms = "FizzBuzz" : bz
    | mo = "Fizz" : bz
    | ms = "Buzz" : bz
    | otherwise = (show (x !! 0)) : bz
    where 
        bz = fizzBuzz ((x !! 0 + 1) : tail x)
        mo = x !! 0 `mod` x !! 1 == 0
        ms = x !! 0 `mod` x !! 2 == 0

rpc x = (read x :: Int)

main = do
    p <- getLine
    putStr $ unlines $ fizzBuzz (1: map (rpc) (words p))