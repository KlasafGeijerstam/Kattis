from collections import deque
n, w = map(int, input().split())
tree = {0: [0, w, [], w]}
for i in range(1, n + 1):
    d, c, u = map(int, input().split())
    tree[i] = [u, c, [], 0]
    tree[d][2].append(i)

required = w
queue = deque()
queue.append(0)

while queue:
    current = queue.popleft()
    current, capacity, edges, parent_req = tree[current]
    for neighbor in edges:
        ncurrent, ncapacity, nneigh, _ = tree[neighbor]
        if ncapacity < parent_req:
            cur_req = (parent_req - ncapacity) + (ncapacity - ncurrent)
        else:
            cur_req = ncapacity - ncurrent

        tree[neighbor][3] = cur_req
        required = min(required, cur_req)

        queue.append(neighbor)

print(required)