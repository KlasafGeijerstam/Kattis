using System;
using System.Collections.Generic;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            for (int i = int.Parse(Console.ReadLine()); i > 0; i--)
            {
                var r = Console.ReadLine().Split();
                var x = new List<float>();
                var y = new List<float>();
                for (int j = 1; j < r.Length - 1; j += 2)
                {
                    x.Add(float.Parse(r[j]));
                    y.Add(float.Parse(r[j + 1]));
                }

                float p1 = 0, p2 = 0;
                for (int j = 0; j < x.Count - 1; j++)
                {
                    p1 += x[j] * y[j + 1];
                    p2 += y[j] * x[j + 1];
                }
                p1 += x[x.Count - 1] * y[0];
                p2 += y[x.Count - 1] * x[0];
                Console.WriteLine(0.5 * (p1 - p2));
            }
        }
    }
}
