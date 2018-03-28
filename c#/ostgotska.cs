using System;
using System.Collections.Generic;
using System.Linq;

class Program
{
    static void Main(string[] args)
    {
        var w = Console.ReadLine().Split();
        Console.WriteLine(w.Count(x => x.Contains("ae")) >= 0.4*w.Length ? "dae ae ju traeligt va" : "haer talar vi rikssvenska");
    }
}
