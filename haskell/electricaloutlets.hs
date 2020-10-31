main = interact solve
solve x = foldl (\a b -> a ++ "\n" ++ (calc $ words b)) "" $ tail $ lines x
calc (l:inp) = show $ 1 + (sum $ map rint inp) - rint l
rint x = read x :: Integer
