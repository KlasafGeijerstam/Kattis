using System;
using System.Linq;
using System.Numerics;
using System.Collections.Generic;
using System.IO;
using System.Diagnostics;
using System.Globalization;

namespace Kattis
{

    public class Program
    {
        public static void Main()
        {
            string str;
            while((str = Console.ReadLine()) != null)
            {
                var i = str.Split(' ').Select(x => double.Parse(x,CultureInfo.InvariantCulture)).ToArray();
                if (i.Length == 1 && i[0] == 0)
                    break;

                Console.WriteLine(Math.Pow(Math.Pow(Math.Abs(i[0] - i[2]),i[4]) + Math.Pow(Math.Abs(i[1] - i[3]),i[4]),1.0/i[4]));
            }
        }
    }
}
