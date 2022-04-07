main = interact solve
solve l = if lw > sw + 1 && lh > sh + 1 then "1" else "0"
    where (lw:lh:sw:sh:_) = map (read) $ words l