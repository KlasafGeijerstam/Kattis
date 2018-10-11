from math import log
n, m = map(int, raw_input().split())
print "Your wish is granted!" if m >= log(n, 2) else "You will become a flying monkey!"
