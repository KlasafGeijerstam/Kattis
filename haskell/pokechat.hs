import Data.List.Split

main = interact solve
solve input = map (\i -> s !! (read i - 1)) $ chunksOf 3 l
    where s:l:_ = lines input
