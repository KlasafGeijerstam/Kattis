import Control.Monad
catalans = map (\n -> product [n+2..2*n] `div` product [1..n]) [0..]

gInt = do
    t <- getLine
    return (read t )

main = do
    q <- getLine
    c <- replicateM (read q) gInt 
    mapM_ print [catalans !! x | x <- c]
