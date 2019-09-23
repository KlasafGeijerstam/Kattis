import Data.List
main = interact p

p x = solve x ++ "\n"
solve x 
    | last y == 'e' = y ++ "x" ++ p
    | elem (last y) "aiou" = (init y) ++ "ex" ++ p
    | (last y) == 'x' && (last $ init y) == 'e' = y ++ p
    | otherwise = y ++ "ex" ++ p
where (y:p:_) = words x 
