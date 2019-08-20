input()
print(sum([abs(x) for x in map(int, input().split()) if x < 0]))