print sum([reduce(lambda x,y: x*y, map(float, raw_input().split())) for _ in range(int(raw_input()))])
