from heapq import heappush, heappop
from math import ceil

a, h = map(int, input().split())
n, m = map(int, input().split())

nodes = [[] for _ in range(n)]
dist = [0 for _ in range(n)]
queue = []
heappush(queue, (-h, 0))
visited = set()

for _ in range(m):
    ei, bi, ai, hi = map(int, input().split())
    nodes[ei - 1].append((bi - 1, ai, hi))

while queue:
    health, node = heappop(queue)
    if node not in visited:
        visited.add(node)
        dist[node] = -health
        for to, ap, hp in nodes[node]:

            k1 = ceil(hp / a - 1)
            k2 = ceil(-health / ap)

            if k1 < k2:
                heappush(queue, (-(-health - k1*ap), to))


print(dist[-1] if dist[-1] > 0 else 'Oh no')
