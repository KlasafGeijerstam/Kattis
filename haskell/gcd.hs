main = interact (\l -> show $ foldl1 gcd $ map read $ words l)
