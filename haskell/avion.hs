import Data.List
main = interact solve

solve x = a ++ "\n"
    where l = lines x
          j = [i | i <- [0..4], (isInfixOf "FBI" $ l !! i)] 
          a = if null j then "HE GOT AWAY!" else foldl (\a c -> a ++ " " ++ show (c + 1)) (show $ (head j) + 1) $ tail j 
