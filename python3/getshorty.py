from heapq import heappush, heappop
n, m = map(int, input().split())

while n + m != 0:
    nodes = [[] for _ in range(n)]
    dist = [0 for _ in range(n)]
    queue = []
    heappush(queue, (-1, 0))
    visited = set()

    for _ in range(m):
        l = input().split()
        u, v, w = int(l[0]), int(l[1]), float(l[2])
        nodes[u].append((v, w))
        nodes[v].append((u, w))

    while queue:
        length, u = heappop(queue)
        if u not in visited:
            visited.add(u)
            dist[u] = -length
            for node, distance in nodes[u]:
                heappush(queue, (length * distance, node))

    print("{0:.4f}".format(dist[n-1]))
    n, m = map(int, input().split())
