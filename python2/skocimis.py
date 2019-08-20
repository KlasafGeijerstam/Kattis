x,y,z = map(int, raw_input().split())
print max(abs(x-y),abs(y-z)) - 1