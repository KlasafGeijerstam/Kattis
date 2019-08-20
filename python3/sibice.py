import math

p = input().split(' ')
for n in range(0, int(p[0])):
    print("DA" if float(input()) <= math.sqrt(math.pow(int(p[1]), 2)+math.pow(int(p[2]), 2)) else "NE")
