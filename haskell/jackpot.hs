main = interact solve
solve l = handle_case $ tail $ lines l
handle_case (_:spin_data:rest) = (handle_machine spin_data) ++ "\n" ++ (handle_case rest)
handle_case [] = ""
handle_machine spinners
    | ans > 1000000000 = "More than a billion."
    | otherwise = show ans
    where ans = foldl1 (lcm) $ map (read) $ words spinners