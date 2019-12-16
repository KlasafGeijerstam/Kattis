import itertools
cases = int(input())


for _ in range(cases):
    a1, b1, a2, b2, a3, b3 = map(int, input().split())
    a = (a1, b1)
    b = (a2, b2)
    c = (a3, b3)
    
    boxes = [[(a1, b1), (b1, a1)], [(a2, b2), (b2, a2)], [(a3, b3), (b3, a3)]]
    myman = 10000000000000000000000000000000000000
    for bconf in itertools.permutations(boxes):
        for b1 in bconf[0]:
            for b2 in bconf[1]:
                for b3 in bconf[2]:

                    #B1B2B3
                    width = b1[0] + b2[0] + b3[0]
                    height = max(b1[1], b2[1], b3[1])
                    if width*height < myman:
                        myman = width * height

                    #B3
                    #B1B2
                    width = max(b3[0], b1[0] + b2[0])
                    height = max(b2[1], b1[1] + b3[1])
                    
                    # COLLIDE

                    if b3[0] > b1[0] and b2[1] > b1[1]:
                        pass
                    else:
                        if width*height < myman:
                            myman = width * height


                    #B2
                    #B1B3
                    width = max(b2[0], b1[0] + b3[0])
                    height = max(b3[1], b1[1] + b2[1])

                    if b2[0] > b1[0] and b3[1] > b1[1]:
                        pass
                    else:
                        if width*height < myman:
                            myman = width * height

                    #B3
                    #B2
                    #B1
                    width = max(b2[0], b1[0], b3[0])
                    height = b1[1] + b2[1] + b3[1]
                    if width*height < myman:
                        myman = width * height

    print(myman)
