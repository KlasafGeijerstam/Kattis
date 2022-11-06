main = interact solve
solve input = show $ abs $ (x1 - x2) * (y1 - y2)
    where x1:y1:x2:y2:_ = map read $ words input :: [Float]
