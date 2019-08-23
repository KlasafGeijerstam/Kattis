main = interact solve

solve x = foldl1 (++) (map (tc.(map (read :: String -> Double).words)) $ lines x)

tc (0:0:_) = ""
tc (d:v:_) = (show $ search d v 0 d 100) ++ "\n"

cylinder r h = pi * r * r * h

cone r h = (pi * r * r * h) / 3

search _ _ lo hi 0 = (hi + lo) / 2
search d v lo hi i = if vol > v then search d v dc hi (i - 1) else search d v lo dc (i - 1)
    where bcone  = cone (d / 2) (d / 2)
          scyl   = cylinder (dc / 2) dc
          scone  = cone (dc / 2) (dc/2)
          vol    = (cylinder (d / 2) d) - (2 * (bcone - scone)) - scyl 
        dc = (hi + lo) / 2
