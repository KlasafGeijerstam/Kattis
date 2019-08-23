main = interact solve

solve x = if n > 7 then "satisfactory\n" else "unsatisfactory\n"
    where (n:_) = map (read :: String -> Int) $ words $ head $ lines x
