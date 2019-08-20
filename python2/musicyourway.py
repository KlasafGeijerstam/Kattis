keys = raw_input().split()
songs = [dict(zip(keys, raw_input().split())) for _ in range(int(raw_input()))]
for _ in range(int(raw_input())):
    key = raw_input()
    songs.sort(key= lambda x: x[key])
    print " ".join(keys)
    for c in songs:
        print " ".join([c[k] for k in keys]) 
    print ""
