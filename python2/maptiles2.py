s = raw_input()
zoom = len(s)

xmin = 0
xmax = 2**zoom
ymin = 0
ymax = 2**zoom

for c in s:
    if c == '0':
        xmax -= abs(xmin-xmax)/2
        ymax -= abs(ymin-ymax)/2
    elif c == '1':
        xmin += abs(xmin-xmax)/2
        ymax -= abs(ymin-ymax)/2 
    elif c == '2':
        ymin += abs(ymin-ymax)/2
        xmax -= abs(xmin-xmax)/2
    elif c =='3':
        ymin += abs(ymin-ymax)/2 
        xmin += abs(xmin-xmax)/2
print "%d %s %s" % (zoom, xmax - 1, ymax - 1)
