import Data.List
bp a = [(read (fst a) :: Integer)..(read (snd a) :: Integer)]
kp (a:b:k) 
    | null k = [(a,b)]
    | otherwise = (a,b) : kp k

solve i = show $ length $ nub $ foldl1 (++) $ map (bp) $ kp $ tail $ words i
    
main = do
    interact (solve)
