n = int(raw_input())
x = zip(range(1,n+1),map(int,raw_input().split()))
x.sort(reverse=True, key=lambda y: y[1])
o = x[0][1]
for i in range(1, len(x)):
    o -= x[i][1]
    if o <= 0:
        print " ".join([str(z[0]) for z in x])
        exit(0)
print "impossible"
