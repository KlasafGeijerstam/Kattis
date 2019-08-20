g, s, c = map(int, raw_input().split())

m = g * 3 + s * 2 + c

v = ""
if m >= 8:
    v = "Province or "
elif m >= 5:
    v = "Duchy or "
elif m >= 2:
    v = "Estate or "

if m >= 6:
    v += "Gold"
elif m >= 3:
    v += "Silver"
else:
    v += "Copper"
print v
