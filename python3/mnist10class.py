from random import choice
for i in range(150):
    print(" ".join([choice(["1", "-1"]) for _ in range(51)]))
