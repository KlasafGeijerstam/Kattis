main = interact solve
solve x = show $ if k !! 1 == "*" then (rint $ k !! 0) * (rint $ k !! 2)
               else (rint $ k !! 0) + (rint $ k !! 2)
    where k = lines x          
rint x = read x :: Integer