using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading;

namespace Kattis
{
    public class Program
    {
        struct Vector3
        {
            public int x;
            public int y;
            public int z;

            public Vector3(int x, int y, int z)
            {
                this.x = x;
                this.y = y;
                this.z = z;
            }
        }
        public static void Main(string[] args)
        {
            int[] v;
            int[,,] map;
            
            while ((v = Console.ReadLine().Split().ToList().Select(x => int.Parse(x)).ToArray()).Sum() != 0)
            {
                var current = new List<Vector3>();
                map = new int[v[2], v[0], v[1]];
                for (int y = 0; y < v[0]; y++)
                {
                    for (int z = 0; z < v[1]; z++)
                    {
                        var s = Console.ReadLine();
                        for (int x = 0; x < v[2]; x++)
                        {
                            map[x, y, z] = s[x];
                            if (s[x] == 'S')
                                current.Add(new Vector3(x, y, z));
                        }
                    }
                    Console.ReadLine();
                }
                //Fill
                int len = 'F';
                bool found = false;
                var pq = new List<Vector3>();
                while (current.Count > 0)
                {
                    len++;
                    foreach (var c in current)
                    {
                        for (int x = -1; x < 2; x++)
                            for (int y = -1; y < 2; y++)
                                for (int z = -1; z < 2; z++)
                                {
                                    var n = 0;
                                    if (x != 0)
                                        n++;
                                    if (y != 0)
                                        n++;
                                    if (z != 0)
                                        n++;
                                    if (!(c.x + x >= 0 && c.x + x < map.GetLength(0) && c.y + y >= 0 && c.y + y < map.GetLength(1) && c.z + z >= 0 && c.z + z < map.GetLength(2)))
                                        continue;
                                    if (n > 0 && n < 2 && (map[c.x + x, c.y + y, c.z + z] == '.' || map[c.x + x, c.y + y, c.z + z] == 'E'))
                                    {
                                        if (map[c.x + x, c.y + y, c.z + z] == 'E')
                                        {
                                            found = true;
                                            goto end;
                                        }
                                        map[c.x + x, c.y + y, c.z + z] = len;
                                        pq.Add(new Vector3(c.x + x, c.y + y, c.z + z));
                                    }
                                }
                    }
                    current.Clear();
                    current.AddRange(pq);
                    pq.Clear();
                }
                end:
                if (found)
                    Console.WriteLine($"Escaped in {len-'F'} minute(s).");
                else
                    Console.WriteLine("Trapped!");
            }
        }
    }
}

