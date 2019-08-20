n, x = map(int, input().split())
nums = sorted(map(int, input().split()))
for i in range(1, n + 1):
    if i >= len(nums) or nums[i] + nums[i-1] > x:
        print(i)
        break
