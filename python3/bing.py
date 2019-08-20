class Node:
    def __init__(self):
        self.children = {}
        self.value = 0

def find(node, s):
    for c in s:
        if c in node.children:
            node = node.children[c]
        else:
            return 0
    return node.value

def insert(node, s):
    for c in s:
        if c not in node.children:
            node.children[c] = Node()
        node = node.children[c]
        node.value += 1

n = Node()
for s in [input() for _ in range(int(input()))]:
    print(find(n, s))
    insert(n, s)
