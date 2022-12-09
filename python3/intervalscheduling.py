n = int(input())

intervals = []
for _ in range(n):
    start, end = map(int, input().split())
    intervals.append((start, end))
    
intervals.sort(key=lambda x: x[1])

current_end = 0
interval_count = 0
for start, end in intervals:
    if start >= current_end:
        interval_count += 1
        current_end = end
print(interval_count)