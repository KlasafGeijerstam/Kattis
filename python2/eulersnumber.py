r = 1 
p = 1
for k in range(min(int(input()), 17)):
    r += 1.0 / p
    p *= (k+2)
print "%.16f" % r
