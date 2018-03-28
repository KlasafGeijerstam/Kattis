using System;
using System.Collections.Generic;
using System.Linq;
class Program
{
    static void Main(string[] args)
    {
        var t = Console.ReadLine().Split().Select(int.Parse).ToArray();
        Console.WriteLine(Enumerable.Range(0, t[0]).Count(y => Enumerable.Range(0, t[1]).Where(x =>
        {
            return Console.ReadLine().Substring(1).Count(char.IsUpper) == 0;
        }).Count() == t[1]));
    }
}
