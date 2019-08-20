r, n = map(int, raw_input().split())
rs = set([i for i in range(1, r+1)]) - set([int(raw_input()) for _ in range(n)])
print list(rs)[0] if rs else "too late"