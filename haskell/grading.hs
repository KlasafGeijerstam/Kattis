main = interact solve
solve l
    | g >= a    = "A"
    | g >= b    = "B"
    | g >= c    = "C"
    | g >= d    = "D"
    | g >= e    = "E"
    | otherwise = "F"
    where (grades:score:_) = lines l 
          (a:b:c:d:e:_) = map (read) $ words grades
          g = read score :: Int