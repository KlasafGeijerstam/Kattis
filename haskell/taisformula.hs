chunks [] = []
chunks (time:glucose:rest) = (time, glucose) : chunks rest

slv _ [] = 0
slv (t',v') (c@(t,v):rst) = (((v' + v) / 2) * (t - t') / 1000) + slv c rst

solve i = show $ (slv f rst)
    where (_:dat) = map read (words i) :: [Double]
          (f:rst) = chunks dat

main = do
    interact (solve)
