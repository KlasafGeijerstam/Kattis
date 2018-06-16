using System;

namespace Csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            for (int i = int.Parse(Console.ReadLine()); i > 0; i--)
            {
                var n = Console.ReadLine().Split();
                int i1 = 0, i2 = n[1].Length-1, len = n[1].Length;
                for (int j = 0; j < int.Parse(n[0]); j++)
                {
                    if ((len) / 4 == 0)
                        break;
                    if (j % 2 == 0)
                        i1 += (len) / 4;
                    else
                        i2 -= (len) / 4;
                    len -= len / 4;
                }
                Console.WriteLine(n[1].Substring(i1,len));
            }
        }
    }
}
