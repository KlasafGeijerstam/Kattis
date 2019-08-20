from sys import stdin
m = {'A# minor' : 'Bb minor',
     'A# major' : 'Bb major',
     'C# minor' : 'Db minor',
     'C# major' : 'Db major',
     'D# minor' : 'Eb minor',
     'D# major' : 'Eb major',
     'F# minor' : 'Gb minor',
     'F# major' : 'Gb major',
     'G# minor' : 'Ab minor',
     'G# major' : 'Ab major'}
for k in m.keys():
    m[m[k]] = k
c = 1
for l in stdin:
    print ("Case %d: " + (m[l.strip()] if l.strip() in m else "UNIQUE")) % c
    c += 1
