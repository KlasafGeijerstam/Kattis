from dataclasses import dataclass, field
from typing import Optional

@dataclass
class Round:
    number: int
    server: str
    points_alice: Optional[int] = field(default=None)
    points_bob: Optional[int] = field(default=None)
    seen: bool = field(default=False)
 
table = [
    Round(0, "alice", 0, 0, True),
    Round(1, "bob"),
    Round(2, "bob"),
    Round(3, "alice"),
    Round(4, "alice"),
    Round(5, "bob"),
    Round(6, "bob"),
    Round(7, "alice"),
    Round(8, "alice"),
    Round(9, "bob"),
    Round(10, "bob"),
    Round(11, "alice"),
    Round(12, "alice"),
    Round(13, "bob"),
    Round(14, "bob"),
    Round(15, "alice"),
    Round(16, "alice"),
    Round(17, "bob"),
    Round(18, "bob"),
    Round(19, "alice"),
    Round(20, "alice"),
    Round(21, "bob"),
]

number_of_rounds = int(input())
previous_score = 0
game_over = False
game_over_bob, game_over_alice = 0, 0
for i in range(number_of_rounds):
    server, non_server = map(int, input().split('-'))
    
    total_score = server + non_server

    if total_score > 21 or total_score < previous_score:
        print(f"error {i + 1}")
        break

    if total_score == 0:
        continue
    
    entry = table[total_score]

    if entry.server == "alice":
        alice = server
        bob = non_server
    else:
        bob = server
        alice = non_server

    if game_over and (game_over_alice != alice or game_over_bob != bob):
        print(f"error {i + 1}")
        break

    if entry.seen and (entry.points_alice != alice or entry.points_bob != bob):
        print(f"error {i + 1}")
        break

    entry.seen = True
    entry.points_bob = bob
    entry.points_alice = alice

    if entry.points_alice == 11 or entry.points_bob == 11:
        game_over = True
        game_over_alice = entry.points_alice
        game_over_bob = entry.points_bob

    previous_entry = None
    for index in range(total_score - 1, -1, -1):
        if table[index].seen:
            previous_entry = table[index]
            break
    
    diff_alice = alice - previous_entry.points_alice
    diff_bob = bob - previous_entry.points_bob
    total_diff = diff_alice + diff_bob
    round_diff = entry.number - previous_entry.number
    
    if diff_alice > round_diff or diff_bob > round_diff or total_diff != round_diff:
        print(f"error {i + 1}")
        break
    
    previous_score = total_score
else:
    print("ok")