using System;
using System.Linq;
using System.Numerics;
using System.Collections.Generic;
using System.IO;
using System.Diagnostics;

namespace Kattis
{

    public class Program
    {
        static int[] ar;
        public static void Main()
        {
            ar = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
            while (ar[0] != 1 || ar[1] != 2 || ar[2] != 3 || ar[3] != 4 || ar[4] != 5)
            {
                oneSort();
                if (changed)
                {
                    var str = "";
                    foreach (var i in ar)
                        str += " " + i;
                    Console.WriteLine(str.Substring(1));
                }
            }
        }
        static int step = 0;
        static bool changed = false;
        static void oneSort()
        {
            changed = false;
            if(ar[step] > ar[step + 1])
            {
                int tmp = ar[step];
                ar[step] = ar[step + 1];
                ar[step + 1] = tmp;
                changed = true;
            }
            step++;
            if (step == 4)
                step = 0;
        }
    }
}
