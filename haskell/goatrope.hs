main = do
    c <- getLine
    let d = map (read :: String->Float) $ words c
    print $ dst (d !! 0, d !! 1) (d !! 2, d !! 3, d !! 4, d !! 5)
dst (p1,p2) (x1,y1,x2,y2) = sqrt $ (maximum [x1 - p1, 0, p1 - x2])**2 + (maximum [y1 - p2, 0, p2 - y2])**2  
