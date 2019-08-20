rests = [[input() for _ in range(1 + int(input()))] for _ in range(int(input()))]
r = [r[0] for r in rests if 'pancakes' in r[1:] and 'pea soup' in r[1:]]
print('Anywhere is fine I guess' if not r else r[0])