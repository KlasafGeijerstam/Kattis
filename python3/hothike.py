n = int(input())
nums = list(map(int, input().split()))

prev = 1000000000
for i in range(2, len(nums)):
    if max(nums[i - 2], nums[i]) < prev:
        index = i - 2
        prev = max(nums[i - 2], nums[i])

print(f'{index + 1} {prev}')
