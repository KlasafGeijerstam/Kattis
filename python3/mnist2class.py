from random import choice
for i in range(30):
    print(" ".join([choice(["1", "-1"]) for _ in range(51)]))