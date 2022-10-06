main = interact solve

solve i = show $ 4 + 2 * x * (x - 1)
    where x = (read i) `div` 4
