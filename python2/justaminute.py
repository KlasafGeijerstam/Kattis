k = [map(int,raw_input().split()) for _ in range(int(raw_input()))]
y = (sum([x[1] for x in k]) / float(60)) / float(sum([x[0] for x in k]))
print ("%.10f" % y) if y > 1.0 else "measurement error"
