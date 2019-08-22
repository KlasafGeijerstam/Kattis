from collections import deque
n, m = map(int, input().split())
field = [list(map(int, input())) for _ in range(n)]
inf = 10000000000000
dist = [[inf for _ in range(m)] for _ in range(n)]
dist[0][0] = 0


def inr(y, x):
    return y < n and y >= 0 and x < m and x >= 0


queue = deque([(0, 0)])
while queue:
    y, x = queue.popleft()
    cur = field[y][x]
    s = dist[y][x]

    if inr(y, x + cur):
        if s + 1 < dist[y][x + cur]:
            dist[y][x + cur] = s + 1
            queue.append((y, x + cur))
    if inr(y, x - cur):
        if s + 1 < dist[y][x - cur]:
            dist[y][x - cur] = s + 1
            queue.append((y, x - cur))
    if inr(y + cur, x):
        if s + 1 < dist[y + cur][x]:
            dist[y + cur][x] = s + 1
            queue.append((y + cur, x))
    if inr(y - cur, x):
        if s + 1 < dist[y - cur][x]:
            dist[y - cur][x] = s + 1
            queue.append((y - cur, x))

print(-1 if dist[-1][-1] == inf else dist[-1][-1])
