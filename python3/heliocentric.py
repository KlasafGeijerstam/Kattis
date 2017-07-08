from sys import stdin
case = 1
for line in stdin:
    line = line.split()
    days = 0
    earth = int(line[0])
    mars = int(line[1])
    while earth != 0 or mars != 0:
        earth = earth + 1
        mars = mars + 1
        days = days + 1
        if earth == 365:
            earth = 0
        if mars == 687:
            mars = 0
    print("Case %d: %d" % (case, days))
    case = case + 1
