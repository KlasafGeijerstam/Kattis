using System;

namespace Kattis
{
    class Program
    {
        public static void Main()
        {
            int n;
            int a, b, c, d, e;
            string[] s;
            n = int.Parse(Console.ReadLine());
            for (int i = 0; i < n; i++)
            {
                s = Console.ReadLine().Split();
                a = (s[0] != "?" ? int.Parse(s[0]) : -1);
                b = (s[1] != "?" ? int.Parse(s[1]) : -1);
                c = (s[2] != "?" ? int.Parse(s[2]) : -1);
                d = (s[3] != "?" ? int.Parse(s[3]) : (a == -1 ? 0 : -1));
                e = (s[4] != "?" ? int.Parse(s[4]) : -1);

                for (int bm = (b == -1 ? 0 : b); bm <= (b == -1 ? 100 : b); bm++)
                    for (int cm = (c == -1 ? 0 : c); cm <= (c == -1 ? 100 : c); cm++)
                    {
                        int am = a, dm = d, em = e;
                        if (a == -1)
                            am = bm + cm + d;
                        else if (d == -1)
                            dm = a - bm - cm;

                        if (e == -1)
                            em = 3 * bm + cm;
                        if (am == bm + cm + dm && 3 * bm + cm == em && am >= 0 && am <= 100 && dm >= 0 && dm <= 100)
                            Console.WriteLine($"{am} {bm} {cm} {dm} {em}");
                    }
            }
        }
    }
}
