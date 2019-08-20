from heapq import heappush, heappop
n, m, q, s = map(int, input().split())

while n + m + q + s  != 0:
    nodes = [[] for _ in range(n)]
    dist = [1000000000 for _ in range(n)]
    dist[s] = 0
    queue = []
    heappush(queue, (0, s))
    visited = set()

    for u, v, w in [list(map(int, input().split())) for _ in range(m)]:
        nodes[u].append((v, w))

    while queue:
        length, u = heappop(queue)
        if u not in visited:
            visited.add(u)
            for node, distance in nodes[u]:
                a = length + distance
                if a < dist[node]:
                    dist[node] = a
                if node not in visited:
                    heappush(queue, (dist[node], node))

    for t in [int(input()) for _ in range(q)]:
        print("Impossible" if dist[t] == 1000000000 else dist[t])
    print()
    n, m, q, s = map(int, input().split())
