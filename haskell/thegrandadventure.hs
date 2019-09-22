main = interact solve

solve (_:x) = foldl1 (\y p -> y ++ if move p [] then "YES\n" else "NO\n") $ lines x 

move [] (x:_) = False
move [] [] = True
move ('.':r) b = move r b
move (c:m) ba
    | elem c "|*$" = move m $ c:ba
    | null ba = False 
    | c == 't' && b == '|' = move m r
    | c == 'j' && b == '*' = move m r
    | c == 'b' && b == '$' = move m r
    | otherwise = False
    where r = tail ba
          b = head ba
