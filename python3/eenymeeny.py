nwords = len(input().split())
people = [input() for _ in range(int(input()))]

a = []
b = []
asd = True
i = 0
while people:
    i = (nwords + i -1) % len(people)
    if asd:
        a.append(people[i])
        del people[i]
    else:
        b.append(people[i])
        del people[i]
    asd = not asd
print(len(a))
print("\n".join(a))

print(len(b))
print("\n".join(b))

