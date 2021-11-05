main = interact solve

solve a = if odd c then "still running" else show $ calc r
    where
        c:r = map (read) $ lines a :: [Integer]

calc [] = 0
calc (s:e:r) = (e - s) + calc r
