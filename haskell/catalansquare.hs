cats = scanl (\c n -> c * 2 * (2 * n - 1) `div` (n + 1)) 1 [1 ..]
main = interact solve
solve x = show $ sum $ zipWith (*) c $ reverse c
    where c = take ((read x :: Int) + 1) cats
