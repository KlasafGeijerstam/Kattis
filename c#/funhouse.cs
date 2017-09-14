using System;
using System.Linq;

namespace Kattis
{
    public enum Dir { U = 0, R = 1, D = 2, L = 3 }
    struct Point
    {
        public int x, y;
        public Point(int x, int y)
        {
            this.x = x;
            this.y = y;
        }
    }
    class Program
    {
        public static void Main()
        {
            
            for(int pase = 1; ; pase++)
            {
                var wl = Console.ReadLine().Split().Select(x => int.Parse(x)).ToArray();
                int w = wl[0], l = wl[1];
                if (w + l == 0)
                    break;
                string[] arr = new string[l];
                for (int i = 0; i < l; i++)
                    arr[i] = Console.ReadLine();

                int sx = 0, sy = 0;
                for(int y = 0; y < l; y++)
                    for(int x = 0; x < w; x++)
                        if(arr[y][x] == '*')
                        {
                            sx = x; sy = y;
                            goto pep;
                        }
                pep:
                Dir d;
                if (sx == 0) d = Dir.R;
                else if (sy == 0) d = Dir.D;
                else if (sx == w - 1) d = Dir.L;
                else d = Dir.U;

                while(true)
                {
                    d = Next(d, arr[sy][sx]);
                    var m = Move(d);
                    sy += m.y;
                    sx += m.x;
                    if (arr[sy][sx] == 'x')
                        break;
                }
                Console.WriteLine("HOUSE " + pase);
                for (int y = 0; y < l; y++)
                {
                    for (int x = 0; x < w; x++)
                        Console.Write(y == sy && x == sx ? '&' : arr[y][x]);
                    Console.WriteLine();
                }
            }   
        }
        static Dir Next(Dir d, char c)
        {
            if (c == '\\' || c == '/')
            {
                if(d == Dir.U || d == Dir.D)
                    if (c == '/') return Increment(d);
                    else return Decrement(d);
                if(d ==  Dir.R || d == Dir.L)
                    if (c == '/') return Decrement(d);
                    else return Increment(d);
                return d;
            }
            else return d;
        }

        static Point Move(Dir d)
        {
            if (d == Dir.U) return new Point(0, -1);
            else if (d == Dir.R) return new Point(1, 0);
            else if (d == Dir.D) return new Point(0, 1);
            else return new Point(-1, 0);
        }

        static Dir Increment(Dir d)
        {
            if (d != Dir.L) return d + 1;
            else return Dir.U;
        }

        static Dir Decrement(Dir d)
        {
            if (d != Dir.U) return d - 1;
            else return Dir.L;
        }
    }
}
