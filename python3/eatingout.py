m, a, b, c = map(int, input().split())
print('possible' if a + b + c <= m+m else 'impossible')
