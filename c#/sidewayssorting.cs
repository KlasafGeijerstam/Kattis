using System;
using System.Collections.Generic;
using System.Linq;
class Program
{
    static void Main(string[] args)
    {
        while(true)
        {
            var c = Console.ReadLine().Split().Select(int.Parse).ToArray();
            if (c.Sum() == 0)
                break;
            var l = new List<string>(Enumerable.Range(0, c[1]).Select(x => ""));
            Enumerable.Range(0, c[0]).ToList().ForEach(x => { var z = Console.ReadLine(); Enumerable.Range(0, c[1]).ToList().ForEach(y => l[y] += z[y]); });
            l.Sort(StringComparer.InvariantCultureIgnoreCase);
            Enumerable.Range(0, c[0]).ToList().ForEach(x => { Enumerable.Range(0, c[1]).ToList().ForEach(y => Console.Write(l[y][x]));Console.WriteLine(); });
            Console.WriteLine();
        }
    }
}
